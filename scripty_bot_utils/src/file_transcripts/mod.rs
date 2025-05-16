mod consts;
mod error;
mod handler;
mod message_updater;
mod raw_pcm_conversions;
mod state;

use std::{fmt::Write, time::Duration};

use poise::CreateReply;
use scripty_i18n::LanguageIdentifier;
use serenity::{
	builder::CreateAttachment,
	model::{channel::Message, id::UserId},
};

pub use self::{
	error::FileTranscriptError,
	handler::FileTranscriptionHandler,
	message_updater::MessageUpdater,
	state::{TranscriptResult, TranscriptResultEnum, TranscriptionState, TranscriptionStateEnum},
};
use crate::Error;

pub async fn transcribe_generic_message(
	target_msg: Message,
	mut reply_msg: MessageUpdater<'_>,
	invoking_user: Option<UserId>,
	resolved_language: LanguageIdentifier,
) -> Result<(), Error> {
	let target_msg_id = target_msg.id;
	let target_guild_id = target_msg.guild_id;

	let (tx, mut rx) = tokio::sync::mpsc::channel(5);
	let file_transcript = FileTranscriptionHandler::new(target_msg, tx, invoking_user, false);

	let mut transcript_join_handle = tokio::spawn(file_transcript.run_transcription());

	let final_transcript_result = loop {
		let TranscriptionState { filename, state } = tokio::select! {
			r = &mut transcript_join_handle => {
				break r;
			}
			state = rx.recv() => {
				if let Some(state) = state {
					state
				} else {
					let ret = tokio::time::timeout(Duration::from_secs(10), transcript_join_handle).await;
					match ret {
						Ok(r) => break r,
						Err(_) => {
							error!(%target_msg_id, "timed out waiting for final result after stream disappeared");
							reply_msg.edit_message(
								CreateReply::new().content(format_message!(
									resolved_language,
									"transcribe-message-timed-out-after-reply",
									msgId: target_msg_id.to_string()
								)),
							).await?;

							return Ok(());
						}
					}
				}
			}
		};

		let filename = filename.to_string();
		let fmt_string = match state {
			TranscriptionStateEnum::Downloading { file_size } => {
				format_message!(resolved_language, "transcribe-message-downloading-file", filename: filename, fileSize: format!("{:.3}", file_size))
			}
			TranscriptionStateEnum::Probing => {
				format_message!(resolved_language, "transcribe-message-probing-file", filename: filename)
			}
			TranscriptionStateEnum::Transcoding { file_length } => {
				format_message!(resolved_language, "transcribe-message-transcoding-file", filename: filename, fileLength: format!("{:.3}", file_length))
			}
			TranscriptionStateEnum::Transcribing { file_length } => {
				format_message!(resolved_language, "transcribe-message-transcribing-file", filename: filename, fileLength: format!("{:.3}", file_length))
			}
		};

		reply_msg
			.ensure_message(format_message!(
				resolved_language,
				"transcribe-message-initial-reply"
			))
			.await?;
		reply_msg
			.edit_message(CreateReply::new().content(fmt_string))
			.await?;
	};

	let final_transcripts = final_transcript_result??;

	let verbose = if let Some(guild_id) = target_guild_id {
		let db = scripty_db::get_db();
		sqlx::query!(
			"SELECT be_verbose FROM guilds WHERE guild_id = $1",
			guild_id.get() as i64
		)
		.fetch_optional(db)
		.await?
		.is_some_and(|r| r.be_verbose)
	} else {
		// always be verbose in DMs
		true
	};

	send_final_transcript_edit(final_transcripts, reply_msg, &resolved_language, verbose).await
}

async fn send_final_transcript_edit(
	mut final_transcripts: Vec<TranscriptResult>,
	mut reply_msg: MessageUpdater<'_>,
	resolved_language: &LanguageIdentifier,
	verbose: bool,
) -> Result<(), Error> {
	let edit_msg = if final_transcripts.is_empty() {
		CreateReply::new().content(format_message!(
			resolved_language,
			"transcribe-message-no-results"
		))
	} else if final_transcripts.len() == 1
		&& let Some(transcript_result) = final_transcripts.pop()
	{
		// single transcript
		let mut msg_content = String::new();
		let edit_msg = format_transcript_msg(
			CreateReply::new(),
			&mut msg_content,
			transcript_result,
			resolved_language,
			true,
			verbose,
		);
		edit_msg.content(msg_content)
	} else {
		let mut edit_msg = CreateReply::new();
		let mut msg_content = String::new();
		for transcript in final_transcripts {
			edit_msg = format_transcript_msg(
				edit_msg,
				&mut msg_content,
				transcript,
				resolved_language,
				false,
				verbose,
			);
		}
		edit_msg.content(msg_content)
	};

	reply_msg.edit_message(edit_msg).await?;

	Ok(())
}

fn format_transcript_msg<'a>(
	msg: CreateReply<'a>,
	msg_content: &mut String,
	TranscriptResult { filename, state }: TranscriptResult,
	resolved_language: &LanguageIdentifier,
	is_single: bool,
	verbose: bool,
) -> CreateReply<'a> {
	match state {
		TranscriptResultEnum::Success {
			file_length,
			took,
			transcript,
		} if transcript.len() <= 1800 && is_single => {
			// this is the sole transcript, and it is under 1800 characters, so we can send it inline
			if verbose {
				msg_content.push_str(&format_message!(
					resolved_language,
					"transcribe-message-inline-header"
				));
				msg_content.push('\n');
			}
			for line in transcript.split_inclusive('\n') {
				msg_content.push_str("> ");
				msg_content.push_str(line);
			}

			msg_content.push('\n');

			let took_f64 = took.as_secs_f64();
			let unusually_long = took_f64 > file_length;
			if verbose {
				msg_content.push_str(&format_message!(
					resolved_language,
					"transcribe-message-time-taken-single-file",
					timeTaken: format!("{:.3}", took.as_secs_f64()),
					fileLength: format!("{:.3}", file_length)
				));
				msg_content.push('\n');
				if unusually_long {
					msg_content.push_str(&format_message!(
						resolved_language,
						"transcribe-message-unusually-long"
					));
				}
			} else {
				write!(
					msg_content,
					"-# {:.3} ({:.3}){}",
					took.as_secs_f64(),
					file_length,
					if unusually_long { " ⚠" } else { "" }
				)
				.expect("writing to string should be infallible")
			}

			msg
		}

		TranscriptResultEnum::Success {
			file_length,
			took,
			transcript,
		} => {
			// either this is not the sole transcript or it is over 1800 characters,
			// so it must be added as a file
			msg_content.push('\n');
			msg_content.push_str(&format_message!(
				resolved_language,
				"transcribe-message-time-taken-named-file",
				filename: filename.to_string(),
				timeTaken: format!("{:.3}", took.as_secs_f64()),
				fileLength: format!("{:.3}", file_length)
			));

			msg.attachment(CreateAttachment::bytes(
				transcript.into_bytes(),
				format!("transcript_{}.txt", filename),
			))
		}

		TranscriptResultEnum::EmptyTranscript { file_length, took } => {
			msg_content.push('\n');
			msg_content.push_str(&format_message!(
				resolved_language,
				"transcribe-message-no-transcript",
				filename: filename.to_string(),
				fileLength: format!("{:.3}", file_length),
				took: format!("{:?}", took)
			));

			msg
		}

		TranscriptResultEnum::FileTooLong {
			file_length,
			max_file_length,
		} => {
			msg_content.push('\n');
			msg_content.push_str(&format_message!(
				resolved_language,
				"transcribe-message-too-long",
				filename: filename.to_string(),
				maxFileLength: format!("{:.3}", max_file_length),
				fileLength: format!("{:.3}", file_length)
			));

			msg
		}

		TranscriptResultEnum::VideoNeedsPremium => {
			msg_content.push('\n');
			msg_content.push_str(&format_message!(
				resolved_language,
				"transcribe-message-video-needs-premium",
				filename: filename.to_string(),
			));

			msg
		}

		TranscriptResultEnum::Error { error } => {
			msg_content.push('\n');
			msg_content.push_str(&format_message!(
				resolved_language,
				"transcribe-message-result-error",
				filename: filename.to_string(),
				error: error.to_string()
			));

			msg
		}

		TranscriptResultEnum::MalformedInput { cause } => {
			msg_content.push('\n');
			msg_content.push_str(&format_message!(
				resolved_language,
				"transcribe-message-malformed-input",
				filename: filename.to_string(),
				error: cause.to_string()
			));

			msg
		}

		TranscriptResultEnum::DiscordError { error } => {
			msg_content.push('\n');
			msg_content.push_str(&format_message!(
				resolved_language,
				"transcribe-message-discord-error",
				filename: filename.to_string(),
				error: format!("{:?}", error)
			));

			msg
		}
	}
}

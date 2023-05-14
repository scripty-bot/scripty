#[macro_use]
extern crate tracing;

use std::collections::HashMap;

#[tokio::main]
async fn main() {
	scripty_config::load_config(
		&std::env::args()
			.nth(1)
			.expect("first arg should be path to bot config"),
	);

	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

	let pg_connect_options = database_url
		.parse::<sqlx::postgres::PgConnectOptions>()
		.expect("database URL invalid");
	info!("connecting to DB");

	let db = sqlx::postgres::PgPoolOptions::new()
		.connect_with(pg_connect_options)
		.await
		.expect("failed to connect to DB");

	info!("connected to DB");

	////////////////////////////////////////////////////////////////////////////
	// writing messages out
	////////////////////////////////////////////////////////////////////////////
	info!("fetching messages");
	let res = sqlx::query!("SELECT * FROM message_store")
		.fetch_all(&db)
		.await
		.expect("failed to run query");
	info!("found {} messages", res.len());

	let mut output = Vec::with_capacity(res.len());
	for row in res {
		let encrypted_content = row.message_content;
		let mut nonce: Vec<u8> = row.nonce;

		let mut nonce_array = [0; 12];
		nonce.truncate(12);
		nonce_array.copy_from_slice(&nonce);

		let decrypted_content =
			match scripty_data_storage::decrypt_bytes(&encrypted_content, nonce_array)
				.map(|x| String::from_utf8_lossy(&x).to_string())
			{
				Ok(decrypted_content) => decrypted_content,
				Err(e) => {
					error!("Error decrypting message: {}", e);
					continue;
				}
			};
		output.push(decrypted_content);
	}
	// write output to a CSV file
	info!("writing messages");

	let mut writer = csv::Writer::from_path("output.csv").expect("failed to open output.csv");
	for line in output {
		writer.write_record(&[&line]).expect("failed to write line");
	}

	////////////////////////////////////////////////////////////////////////////
	// writing audio data out
	////////////////////////////////////////////////////////////////////////////
	info!("fetching audio data");
	let res = sqlx::query!("SELECT * FROM audio_store")
		.fetch_all(&db)
		.await
		.expect("failed to run query");
	info!("found {} audio data", res.len());

	let mut output = HashMap::with_capacity(res.len());
	for row in res {
		let audio_data: Vec<u8> = row.audio_data; // type bytea (WAV encoded PCM 16-bit mono)
		let transcript: String = row.transcript; // type text
		let transcript_language: String = row.transcript_language; // type text (ISO 639-1 language code)
		let id: i32 = row.id; // type bigint

		output
			.entry(transcript_language)
			.or_insert_with(|| Vec::with_capacity(1))
			.push((id, audio_data, transcript));
	}
	info!("writing audio data");

	// write output to a directory tree
	// root directory is language code
	// each directory contains a CSV file that maps transcripts to audio filenames
	// each audio file is in the form {id}.wav

	for (language, audio_data) in output.into_iter() {
		std::fs::create_dir(&language).expect("failed to create directory");
		let mut writer = csv::Writer::from_path(format!("{}/transcripts.csv", language))
			.expect("failed to open transcripts.csv");
		for (id, audio_data, transcript) in audio_data {
			writer
				.write_record(&[&transcript, &format!("{}.wav", id)])
				.expect("failed to write line");
			std::fs::write(format!("{}/{}.wav", language, id), &audio_data)
				.expect("failed to write file");
		}
	}

	////////////////////////////////////////////////////////////////////////////
}

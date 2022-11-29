use crate::Data;
use backtrace::Backtrace;
use poise::{Context, CreateReply, FrameworkError};
use scripty_audio_handler::JoinError;
use serenity::builder::{
    CreateAttachment, CreateEmbed, CreateEmbedAuthor, CreateMessage, ExecuteWebhook,
};
use serenity::model::channel::ChannelType;
use serenity::model::webhook::Webhook;
use serenity::prelude::SerenityError;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::Write;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    bt: Backtrace,
    err: ErrorEnum,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorEnum {
    Serenity(serenity::Error),
    InvalidChannelType {
        expected: ChannelType,
        got: ChannelType,
    },
    Db(sqlx::Error),
    ExpectedGuild,
    Join(JoinError),
    ManualError,
    Redis(scripty_redis::redis::RedisError),
    RedisPool(scripty_redis::PoolError),
    Custom(String),
}

#[allow(dead_code)]
impl Error {
    #[inline]
    pub fn serenity(err: serenity::Error) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Serenity(err),
        }
    }

    #[inline]
    pub fn invalid_channel_type(expected: ChannelType, got: ChannelType) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::InvalidChannelType { expected, got },
        }
    }

    #[inline]
    pub fn db(err: sqlx::Error) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Db(err),
        }
    }

    #[inline]
    pub fn expected_guild() -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::ExpectedGuild,
        }
    }

    #[inline]
    pub fn join(err: JoinError) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Join(err),
        }
    }

    #[inline]
    pub fn manual() -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::ManualError,
        }
    }

    #[inline]
    pub fn redis(err: scripty_redis::redis::RedisError) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Redis(err),
        }
    }

    #[inline]
    pub fn redis_pool(err: scripty_redis::PoolError) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::RedisPool(err),
        }
    }

    #[inline]
    pub fn custom(err: String) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Custom(err),
        }
    }

    #[inline]
    pub fn backtrace(&mut self) -> &Backtrace {
        self.bt.resolve();
        &self.bt
    }

    /// Whether a command handler should actually handle this error and note it for the user,
    /// or if it should silently be ignored.
    ///
    /// Returns true if the error should be handled, false if it should be ignored.
    pub fn should_handle(&self) -> bool {
        match &self.err {
            ErrorEnum::Serenity(SerenityError::Http(
                serenity::http::HttpError::UnsuccessfulRequest(serenity::http::ErrorResponse {
                    error: serenity::http::DiscordJsonError { code, .. },
                    ..
                }),
            )) if code == &10062 => {
                // ignore unknown interaction errors
                false
            }
            _ => true,
        }
    }

    /// If this is a user error. If it is, this should be handled in a different way to
    /// return a user-friendly error message.
    ///
    /// Returns true if this is a user error, false if it is not.
    pub fn is_user_error(&self) -> bool {
        matches!(
            &self.err,
            ErrorEnum::ExpectedGuild | ErrorEnum::InvalidChannelType { .. }
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use self::ErrorEnum::*;
        let res: Cow<str> = match &self.err {
            Serenity(e) => format!("Discord/wrapper returned an error: {}", e).into(),
            InvalidChannelType { expected, got } => format!(
                "Got an invalid channel type {:?} when expected {:?}",
                got, expected
            )
            .into(),
            Db(e) => format!("Database returned an error: {:?}", e).into(),
            // _ => "an unknown error happened".into(),
            ExpectedGuild => "expected this to be in a guild".into(),
            Join(e) => format!("failed to join VC: {}", e).into(),
            ManualError => "manual error".into(),
            Redis(e) => format!("Redis returned an error: {}", e).into(),
            RedisPool(e) => format!("Redis pool returned an error: {}", e).into(),
            Custom(e) => format!("Custom error: {}", e).into(),
        };
        f.write_str(res.as_ref())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::ErrorEnum::*;
        match &self.err {
            Serenity(e) => Some(e),
            InvalidChannelType { .. } => None,
            Db(e) => Some(e),
            ExpectedGuild => None,
            Join(e) => Some(e),
            ManualError => None,
            Redis(e) => Some(e),
            RedisPool(e) => Some(e),
            Custom(_) => None,
        }
    }
}

impl From<serenity::Error> for Error {
    #[inline]
    fn from(e: serenity::Error) -> Self {
        Self {
            err: ErrorEnum::Serenity(e),
            bt: Backtrace::new(),
        }
    }
}

impl From<sqlx::Error> for Error {
    #[inline]
    fn from(e: sqlx::Error) -> Self {
        Self {
            err: ErrorEnum::Db(e),
            bt: Backtrace::new(),
        }
    }
}

impl From<scripty_audio_handler::Error> for Error {
    #[inline]
    fn from(e: scripty_audio_handler::Error) -> Self {
        match e {
            scripty_audio_handler::Error::Join(e) => Self::join(e),
            scripty_audio_handler::Error::Database(e) => Self::db(e),
            scripty_audio_handler::Error::Serenity(e) => Self::serenity(e),
        }
    }
}

impl From<scripty_redis::redis::RedisError> for Error {
    #[inline]
    fn from(e: scripty_redis::redis::RedisError) -> Self {
        Self {
            err: ErrorEnum::Redis(e),
            bt: Backtrace::new(),
        }
    }
}

impl From<scripty_redis::PoolError> for Error {
    #[inline]
    fn from(e: scripty_redis::PoolError) -> Self {
        Self {
            err: ErrorEnum::RedisPool(e),
            bt: Backtrace::new(),
        }
    }
}

impl From<String> for Error {
    #[inline]
    fn from(e: String) -> Self {
        Self {
            err: ErrorEnum::Custom(e),
            bt: Backtrace::new(),
        }
    }
}

pub async fn on_error(error: FrameworkError<'_, Data, crate::Error>) {
    info!("handling error event");
    #[allow(unreachable_patterns)]
    match error {
        FrameworkError::Setup { error, .. } => panic!("error during bot init: {}", error),
        FrameworkError::Listener { error, event, .. } => {
            error!(
                "error in listener for event {}: {}",
                event.snake_case_name(),
                error
            )
        }
        FrameworkError::Command { error, ctx } => {
            if !error.should_handle() {
                return;
            }

            let cmd_name = &ctx.command().qualified_name;

            // if this is a 403 error, it's probably because the bot doesn't have permissions
            match error.err {
                ErrorEnum::Serenity(serenity::Error::Http(
                    serenity::http::HttpError::UnsuccessfulRequest(serenity::http::ErrorResponse {
                        status_code,
                        ..
                    }),
                )) if status_code == http::StatusCode::FORBIDDEN => {
                    send_err_msg(
                        ctx,
                        format!("Missing permissions for {}!", cmd_name),
                        "I tried doing something (not sure what) but was not allowed to.\
                     Please check my permissions and try again.",
                    )
                    .await;
                }

                _ if error.is_user_error() => {
                    send_err_msg(
                        ctx,
                        format!("Invalid use of {}", cmd_name),
                        error.to_string(),
                    )
                    .await;
                }

                ref e => {
                    send_err_msg(
                        ctx,
                        format!("An error happened while processing {}", cmd_name),
                        format!(
                            "```\n{:?}\n```\nThis has been automatically reported. \
                        Please do not attempt to repeatedly use this command.",
                            e
                        ),
                    )
                    .await;

                    log_error_message(
                        &ctx,
                        error,
                        Some(format!("running command {}", ctx.command().name)),
                    )
                    .await;
                }
            }
        }
        FrameworkError::ArgumentParse { error, input, ctx } => {
            send_err_msg(
                ctx,
                format!(
                    "Invalid arguments while parsing {}",
                    ctx.command().qualified_name
                ),
                match input {
                    Some(input) => {
                        format!(
                            "Failed to parse `{}` because `{}`\n\
                            **Hint:** if you're trying to mention a channel with prefix commands, use its ID, \
                            as they are the most reliable way of doing so.",
                            input,
                            error
                        )
                    },
                    None => format!("{}", error),
                },
            )
            .await;
        }
        FrameworkError::CommandStructureMismatch { description, ctx } => {
            let mut args = String::new();
            for param in &ctx.command.parameters {
                if param.required {
                    write!(&mut args, "<{}> ", param.name)
                        .expect("failed to format string: this is a bug");
                } else {
                    write!(&mut args, "[{}] ", param.name)
                        .expect("failed to format string: this is a bug");
                }
            }

            let msg = CreateMessage::default().embed(
                CreateEmbed::default()
                    .title(format!(
                        "Invalid structure from Discord while parsing {}",
                        ctx.command.qualified_name
                    ))
                    .color((255, 0, 0))
                    .description(format!(
                        "{}\n\n\
                    **Note**: this is a Discord error\n\
                    The only fix for this is to wait for Discord to propagate slash commands, \
                    which can take up to one hour.\n\
                    If you do not want to wait this hour, you should use the prefix commands: \
                    run this command with `~{} {}`.",
                        description, ctx.command.qualified_name, args
                    )),
            );

            let response = ctx
                .interaction
                .channel_id()
                .send_message(&ctx.discord, msg.clone())
                .await;
            if let Err(e) = response {
                warn!("failed to send message while handling error: {}", e);
                let response = ctx
                    .interaction
                    .user()
                    .direct_message(ctx.discord, msg)
                    .await;
                if let Err(e) = response {
                    error!("failed to DM user while handling error: {}", e)
                }
            }
        }
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
        } => {
            send_err_msg(
                ctx,
                format!("Cooldown hit on {}", ctx.command().qualified_name),
                format!(
                    "{:.2} seconds remaining on cooldown",
                    remaining_cooldown.as_secs_f32()
                ),
            )
            .await;
        }
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
        } => {
            send_err_msg(
                ctx,
                format!("I am missing perms to run {}", ctx.command().qualified_name),
                format!("Permissions missing: {}", missing_permissions),
            )
            .await;
        }
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
        } => {
            send_err_msg(
                ctx,
                format!(
                    "You are missing perms to run {}",
                    ctx.command().qualified_name
                ),
                match missing_permissions {
                    Some(p) => Cow::from(format!("Permissions missing: {}", p)),
                    None => Cow::from("I'm not sure what permissions you're missing."),
                },
            )
            .await;
        }
        FrameworkError::NotAnOwner { ctx } => {
            send_err_msg(
                ctx,
                format!(
                    "You are missing perms to run {}",
                    ctx.command().qualified_name
                ),
                "Not an owner of this bot",
            )
            .await;
        }
        FrameworkError::CommandCheckFailed { error, ctx } => {
            send_err_msg(
                ctx,
                format!("A precondition for {} failed", ctx.command().qualified_name),
                match error {
                    Some(e) => Cow::from(format!("{:?}", e.err)),
                    None => Cow::from("no reason provided"),
                },
            )
            .await;
        }
        _ => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("error while handling error: {}", e)
            }
        }
    }
}

async fn send_err_msg(
    ctx: Context<'_, Data, Error>,
    title: impl Into<String>,
    description: impl Into<String>,
) {
    let embed = CreateEmbed::default()
        .title(title)
        .color((255, 0, 0))
        .description(description);

    let response = ctx.send(CreateReply::default().embed(embed.clone())).await;
    if let Err(e) = response {
        warn!("failed to send message while handling error: {}", e);
        let response = ctx
            .author()
            .direct_message(ctx.discord(), CreateMessage::default().embed(embed))
            .await;
        if let Err(e) = response {
            error!("failed to DM user while handling error: {}", e)
        }
    }
}

pub async fn log_error_message(
    ctx: &Context<'_, Data, Error>,
    mut err: Error,
    invocation_context: Option<String>,
) {
    // build embed
    let mut e = CreateEmbed::default();
    // build message
    let mut m = ExecuteWebhook::default();

    if let Some(inv_ctx) = invocation_context {
        e = e.title(format!("Error while {}", inv_ctx));
    } else {
        e = e.title("Error while doing something");
    }

    let fmt_bt = format!("{:#?}", err.backtrace());
    if fmt_bt.len() > 2048 {
        e = e.field("Backtrace", "See attached file", false);
        m = m.add_file(CreateAttachment::bytes(
            fmt_bt.into_bytes(),
            "backtrace.txt",
        ));
    } else {
        e = e.field("Backtrace", &fmt_bt, false);
    }

    e = e.field("Error (debug)", format!("{:?}", err), false);
    e = e.field("Error (display)", err.to_string(), false);

    // cache the cache
    let cache = ctx.discord().cache.clone();

    let (guild_id, guild_name) = if let Some(guild_id) = ctx.guild_id() {
        let guild_name = cache
            .guild(guild_id)
            .map_or_else(|| "unknown guild".to_string(), |g| g.name.clone());

        e = e.field("Guild ID", guild_id.to_string(), false);
        e = e.field("Guild Name", &guild_name, true);

        (Some(guild_id), Some(guild_name))
    } else {
        e = e.field("Guild ID", "None (DM ctx)", false);
        e = e.field("Guild Name", "None (DM ctx)", true);

        (None, None)
    };

    let channel_id = ctx.channel_id();
    e = e.field("Channel ID", channel_id.to_string(), false);

    let author = ctx.author();
    let author_id = author.id;
    let author_name = author.tag();
    let author_pfp = author.face();
    e = e.author(
        CreateEmbedAuthor::new(format!("{} ({})", author_name, author_id)).icon_url(author_pfp),
    );

    m = m.embed(e);

    let cfg = scripty_config::get_config();
    let dctx = ctx.discord();
    let hook = match Webhook::from_url(dctx, &cfg.error_webhook).await {
        Ok(hook) => hook,
        Err(e) => {
            error!("failed to fetch error webhook: {}", e);
            return;
        }
    };
    if let Err(e) = hook.execute(dctx, false, m).await {
        error!("failed to log error to discord: {}", e);
    }

    error!(?guild_id, ?guild_name, %channel_id, %author_id, %author_name, "error while doing something: {}", err);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotConfig {
    pub database: DatabaseConfig,

    /// Supported languages. (ie their ISO 639-1 code, with optional country code)
    pub languages: Vec<String>,

    /// Bot token.
    pub token: String,

    /// Support server invite link.
    pub support_invite: String,

    /// Path to i18n files. Must be available at runtime.
    pub i18n_dir: String,

    /// Authentication tokens for the bot's built-in API. These tokens are global.
    pub api_tokens: Vec<String>,

    /// List of bot owners.
    pub owners: Vec<u64>,

    /// Secret key for encrypting messages.
    ///
    /// Generate a new one with `openssl rand -base64 96`.
    pub secret_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: DatabaseConnection,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DatabaseConnection {
    Tcp(String, u16),
    Unix(String),
}

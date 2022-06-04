#[derive(Serialize, Deserialize, Debug)]
pub struct BotConfig {
    pub database: DatabaseConfig,

    /// Path to a directory containing models.
    ///
    /// This should contain directories specifying the two-letter ISO language code,
    /// which themselves contain actual models and scorers.
    ///
    /// Note that the naming of the model and scorer files does not matter. The only requirements are:
    /// * for models:
    ///   * the filename must end with `.tflite`
    /// * for scorers:
    ///   * the filename must end with `.scorer`
    ///
    /// For example:
    /// ```not_rust
    /// ├─ en
    /// │ ├─ model.tflite
    /// │ └─ en.scorer
    /// └─ fr
    ///   ├─ fr.tflite
    ///   └─ model.scorer
    /// ```
    pub model_dir: String,

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

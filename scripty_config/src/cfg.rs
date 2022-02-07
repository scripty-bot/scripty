#[derive(Serialize, Deserialize, Debug)]
pub struct BotConfig {
    pub database: DatabaseConfig,

    /// What percentage of system cores should be dedicated to speech-to-text?
    ///
    /// The minimum number of threads is constrained to 1 for both.
    pub pct_stt_threads: f32,

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

    pub token: String,

    pub support_invite: String,
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

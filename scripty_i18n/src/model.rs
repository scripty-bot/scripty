#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TranslationStrings {
    /// English: Transcript 1/{}
    pub transcript_count: String,

    /// English: Transcription
    pub transcript_content: String,

    /// English: Confidence
    pub transcript_confidence: String,
}

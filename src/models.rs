use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WordElement {
    pub word: String,
    pub score: i32,
    #[serde(rename = "numSyllables")]
    pub num_syllables: Option<i32>,
    pub defs: Option<Vec<String>>,
}

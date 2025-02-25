use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WordElement {
    pub word: String,
    pub score: i32,
    #[serde(rename = "numSyllables")]
    pub num_syllables: Option<i32>,
    pub defs: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    pub frequency: Option<f64>,
}

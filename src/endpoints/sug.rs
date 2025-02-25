use reqwest::Client;
use crate::utils::build_query_string;
use crate::models::WordElement;
use crate::error::DatamuseError;

#[derive(Debug, Clone)]
pub struct SugEndpoint {
    client: Client,
    s: Option<String>,
    max: Option<u32>,
}

impl SugEndpoint {
    pub fn new(client: Client) -> Self {
        SugEndpoint {
            client,
            s: None,
            max: None,
        }
    }

    pub fn hint_string(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.s = Some(term.to_string());
        new_endpoint
    }

    pub fn max(&self, max: u32) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.max = Some(max);
        new_endpoint
    }

    pub async fn call(&self) -> Result<Vec<WordElement>, DatamuseError> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(s) = &self.s {
            params.push(("s", s.clone()));
        }
        if let Some(max) = &self.max {
            params.push(("max", max.to_string()));
        }

        let query_string = build_query_string(&params);
        let url = format!("https://api.datamuse.com/sug?{}", query_string);

        let response = self.client.get(&url).send().await.map_err(DatamuseError::NetworkError)?;

        if !response.status().is_success() {
            return Err(DatamuseError::ApiError(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }

        let response_text = response.text().await.map_err(DatamuseError::NetworkError)?;
        let mut word_list: Vec<WordElement> = serde_json::from_str(&response_text)
            .map_err(|e| DatamuseError::ParsingError(format!("Failed to parse JSON: {}", e)))?;

        // Process frequency data from tags
        for word in &mut word_list {
            if let Some(tags) = &word.tags {
                for tag in tags {
                    if tag.starts_with("f:") {
                        if let Ok(freq) = tag[2..].parse::<f64>() {
                            word.frequency = Some(freq);
                            break;
                        }
                    }
                }
            }
        }

        Ok(word_list)
    }
}

use reqwest::Client;
use crate::utils::build_query_string;
use crate::models::WordElement;

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

    pub async fn call(&self) -> Result<Vec<WordElement>, Box<dyn std::error::Error>> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(s) = &self.s {
            params.push(("s", s.clone()));
        }
        if let Some(max) = &self.max {
            params.push(("max", max.to_string()));
        }

        let query_string = build_query_string(&params);
        let url = format!("https://api.datamuse.com/sug?{}", query_string);

        let response = self.client.get(&url).send().await?;
        let word_list: Vec<WordElement> = response.json().await?;

        Ok(word_list)
    }
}

use reqwest::Client;

#[derive(Debug, Clone)]
pub struct DatamuseClient {
    client: Client,
}

impl DatamuseClient {
    pub fn new() -> Self {
        DatamuseClient {
            client: Client::new(),
        }
    }

    pub fn words(&self) -> crate::endpoints::words::WordsEndpoint {
        crate::endpoints::words::WordsEndpoint::new(self.client.clone())
    }

    pub fn sug(&self) -> crate::endpoints::sug::SugEndpoint {
        crate::endpoints::sug::SugEndpoint::new(self.client.clone())
    }
}

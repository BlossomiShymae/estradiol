use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Stream {
    url: String,
}

impl Stream {
    pub fn url(&self) -> String {
        self.url.clone()
    }
}

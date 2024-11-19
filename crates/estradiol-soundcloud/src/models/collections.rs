use serde::Deserialize;

use super::resources::Resource;

#[derive(Debug, Deserialize)]
pub struct Collection {
    collection: Vec<Resource>,
    total_results: Option<i64>,
    next_href: Option<String>,
    query_urn: Option<String>,
}

impl Collection {
    pub fn collection(&self) -> Vec<Resource> {
        self.collection.clone()
    }

    pub fn total_results(&self) -> Option<i64> {
        self.total_results.clone()
    }

    pub fn next_href(&self) -> Option<String> {
        self.next_href.clone()
    }

    pub fn query_urn(&self) -> Option<String> {
        self.query_urn.clone()
    }
}

use std::collections::HashMap;

use markup5ever_rcdom::{Node, NodeData};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName};

pub struct Webpage {
    pub headers: HeaderMap,
    pub body: String,
    pub script_nodes: Vec<NodeData>,
    pub meta_nodes: Vec<Node>,
}

impl Webpage {
    pub async fn create_from_url(url: &str) -> Self {
        let client = Client::new();
        let response = client.get(url).send().await.unwrap();
        let headers = response.headers().clone();
        let body = response.text().await.unwrap();

        Webpage::default()
            .with_header_map(headers)
            .with_body(body.as_str())
    }

    pub fn default() -> Self {
        Webpage {
            headers: HeaderMap::new(),
            body: "".to_string(),
            script_nodes: Vec::new(),
            meta_nodes: Vec::new(),
        }
    }

    pub fn with_headers(mut self, headers: HashMap<&str, &str>) -> Self {
        let headers = headers.iter().fold(HeaderMap::new(), |mut acc, (k, v)| {
            acc.insert(
                HeaderName::from_bytes(k.as_bytes()).unwrap(),
                v.parse().unwrap(),
            );
            acc
        });

        self.headers = headers;
        self
    }

    pub fn with_header_map(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn with_script_nodes(mut self, script_nodes: Vec<NodeData>) -> Self {
        self.script_nodes = script_nodes;
        self
    }

    pub fn with_meta_nodes(mut self, meta_nodes: Vec<Node>) -> Self {
        self.meta_nodes = meta_nodes;
        self
    }
}

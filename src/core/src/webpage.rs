use std::collections::HashMap;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use reqwest::{Client, Error};
use reqwest::header::{HeaderMap, HeaderName};

pub struct Webpage {
    pub headers: HeaderMap,
    pub body: String,
    pub script_nodes: HashMap<String, Vec<String>>,
    pub meta_nodes: HashMap<String, Vec<String>>,
}

impl Webpage {
    pub async fn from_url(url: &str) -> Result<Self, Error> {
        let client = Client::new();
        let response = client.get(url).send().await.unwrap();
        let headers = response.headers().clone();
        let body = response.text().await.unwrap();

        let dom = parse_document(RcDom::default(), Default::default()).one(body.clone());

        let mut script_nodes = HashMap::new();
        let mut meta_nodes = HashMap::new();

        Self::extract_nodes(&dom.document, &mut script_nodes, &mut meta_nodes);

        Ok(Webpage::default()
            .with_header_map(headers)
            .with_body(&body)
            .with_script_nodes(script_nodes)
            .with_meta_nodes(meta_nodes))
    }

    fn extract_nodes(
        node: &Handle,
        script_nodes: &mut HashMap<String, Vec<String>>,
        meta_nodes: &mut HashMap<String, Vec<String>>,
    ) {
        match &node.data {
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                if name.local.as_ref() == "script" {
                    for attr in attrs.borrow().iter() {
                        if attr.name.local.as_ref() == "src" {
                            script_nodes
                                .entry("src".to_string())
                                .or_insert(Vec::new())
                                .push(attr.value.to_string());
                        }
                    }
                } else if name.local.as_ref() == "meta" {
                    for attr in attrs.borrow().iter() {
                        meta_nodes
                            .entry(attr.name.local.to_string())
                            .or_insert(Vec::new())
                            .push(attr.value.to_string());
                    }
                }
            }
            _ => {}
        }

        for child in node.children.borrow().iter() {
            Self::extract_nodes(child, script_nodes, meta_nodes);
        }
    }
    pub fn default() -> Self {
        Webpage {
            headers: HeaderMap::new(),
            body: "".to_string(),
            script_nodes: HashMap::new(),
            meta_nodes: HashMap::new(),
        }
    }

    pub fn clone_page(&self) -> Self {
        Webpage {
            headers: self.headers.clone(),
            body: self.body.clone(),
            script_nodes: self.script_nodes.clone(),
            meta_nodes: self.meta_nodes.clone(),
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

    pub fn with_script_nodes(mut self, script_nodes: HashMap<String, Vec<String>>) -> Self {
        self.script_nodes = script_nodes;
        self
    }

    pub fn with_meta_nodes(mut self, meta_nodes: HashMap<String, Vec<String>>) -> Self {
        self.meta_nodes = meta_nodes;
        self
    }
}

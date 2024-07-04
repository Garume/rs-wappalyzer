use std::collections::HashMap;
use std::fs::File;

use rayon::prelude::*;
use rmp_serde::{decode, encode};
use serde::{Deserialize, Deserializer, Serialize};

use crate::wapplyzer_regex::WappalyzerRegex;

#[derive(Serialize, Deserialize, Debug)]
pub struct TechnologyJson {
    pub description: Option<String>,
    pub icon: Option<String>,
    #[serde(deserialize_with = "deserialize_cats")]
    pub cats: Vec<i32>,
    pub cookies: Option<HashMap<String, String>>,
    pub dom: Option<DomElement>,
    pub dns: Option<HashMap<String, StringUnion>>,
    pub js: Option<HashMap<String, String>>,
    pub excludes: Option<StringUnion>,
    pub headers: Option<HashMap<String, String>>,
    pub html: Option<StringUnion>,
    pub text: Option<StringUnion>,
    pub css: Option<StringUnion>,
    pub robots: Option<StringUnion>,
    pub implies: Option<StringUnion>,
    pub requires: Option<StringUnion>,
    #[serde(rename = "requiresCategory")]
    pub requires_category: Option<NumberUnion>,
    pub meta: Option<HashMap<String, StringUnion>>,
    pub probe: Option<HashMap<String, String>>,
    #[serde(rename = "scriptSrc")]
    pub script_src: Option<StringUnion>,
    pub scripts: Option<StringUnion>,
    pub url: Option<StringUnion>,
    pub xhr: Option<StringUnion>,
    pub oss: Option<bool>,
    pub saas: Option<bool>,
    pub pricing: Option<Vec<String>>,
    pub website: String,
}

#[derive(Debug)]
pub struct Technology {
    pub name: String,
    pub cookies: HashMap<String, WappalyzerRegex>,
    pub js: HashMap<String, WappalyzerRegex>,
    pub headers: HashMap<String, WappalyzerRegex>,
    pub html: Vec<WappalyzerRegex>,
    pub script_src: Vec<WappalyzerRegex>,
    pub meta: HashMap<String, WappalyzerRegex>,
}

impl Technology {
    pub fn default(name: &str) -> Self {
        Technology {
            name: name.to_string(),
            cookies: HashMap::new(),
            js: HashMap::new(),
            headers: HashMap::new(),
            html: Vec::new(),
            script_src: Vec::new(),
            meta: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringUnion {
    String(String),
    Vec(Vec<String>),
}

impl From<&str> for StringUnion {
    fn from(s: &str) -> Self {
        StringUnion::String(s.to_string())
    }
}

impl From<Vec<&str>> for StringUnion {
    fn from(v: Vec<&str>) -> Self {
        StringUnion::Vec(v.into_iter().map(|s| s.to_string()).collect())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum NumberUnion {
    String(String),
    Number(i32),
}

impl From<i32> for NumberUnion {
    fn from(n: i32) -> Self {
        NumberUnion::Number(n)
    }
}

impl From<&str> for NumberUnion {
    fn from(s: &str) -> Self {
        NumberUnion::String(s.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum DomElement {
    Simple(String),
    Vec(Vec<String>),
    Complex(DomComplex),
    Nested(HashMap<String, DomElement>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DomComplex {
    pub exists: String,
    pub attributes: HashMap<String, String>,
    pub properties: HashMap<String, String>,
    pub text: String,
}

fn deserialize_cats<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let cats: Vec<Cats> = Deserialize::deserialize(deserializer)?;
    Ok(cats.into_iter().map(|c| c.to_int()).collect())
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Cats {
    String(String),
    Int(i32),
}

impl Cats {
    fn to_int(&self) -> i32 {
        match self {
            Cats::String(s) => s.parse::<i32>().unwrap_or(0),
            Cats::Int(i) => *i,
        }
    }
}

#[derive(Debug)]
pub struct TechnologyCollection {
    pub(crate) data: Vec<Technology>,
}

impl TechnologyCollection {
    pub fn new() -> Self {
        TechnologyCollection { data: Vec::new() }
    }

    pub fn add_item(&mut self, technology: Technology) {
        self.data.push(technology);
    }

    pub fn add_items(&mut self, technologies: Vec<Technology>) {
        self.data.extend(technologies);
    }

    pub fn add_collection(&mut self, collection: TechnologyCollection) {
        self.data.extend(collection.data);
    }

    pub fn save_msgpack(json: &str, save_to: &str) {
        let technology_jsons: HashMap<String, TechnologyJson> = serde_json::from_str(json).unwrap();

        if let Ok(mut file) = File::create(save_to) {
            encode::write(&mut file, &technology_jsons).unwrap();
        } else {
            panic!("Failed to create file");
        }
    }

    pub fn from_msgpack(msgpack: &[u8]) -> Self {
        let technology_jsons: HashMap<String, TechnologyJson> = decode::from_read(msgpack).unwrap();
        let technology_jsons_vec: Vec<(String, TechnologyJson)> =
            technology_jsons.into_iter().collect();
        let data = Self::from_vector(technology_jsons_vec);

        TechnologyCollection { data }
    }
    pub fn from_json(json: &str) -> Self {
        let technology_jsons: HashMap<String, TechnologyJson> = serde_json::from_str(json).unwrap();
        let technology_jsons_vec: Vec<(String, TechnologyJson)> =
            technology_jsons.into_iter().collect();
        let data = Self::from_vector(technology_jsons_vec);

        TechnologyCollection { data }
    }

    fn from_vector(vec: Vec<(String, TechnologyJson)>) -> Vec<Technology> {
        vec.par_iter()
            .map(|(name, technology_json)| {
                let headers: HashMap<String, WappalyzerRegex> = if let Some(headers) =
                    &technology_json.headers
                {
                    headers
                        .iter()
                        .map(|(header, pattern)| (header.clone(), WappalyzerRegex::new(pattern)))
                        .collect()
                } else {
                    HashMap::new()
                };

                let cookies: HashMap<String, WappalyzerRegex> = if let Some(cookies) =
                    &technology_json.cookies
                {
                    cookies
                        .iter()
                        .map(|(cookie, pattern)| (cookie.clone(), WappalyzerRegex::new(pattern)))
                        .collect()
                } else {
                    HashMap::new()
                };

                let html: Vec<WappalyzerRegex> = match &technology_json.html {
                    Some(StringUnion::String(pattern)) => vec![WappalyzerRegex::new(pattern)],
                    Some(StringUnion::Vec(patterns)) => patterns
                        .iter()
                        .map(|pattern| WappalyzerRegex::new(pattern))
                        .collect(),
                    None => vec![],
                };

                let js: HashMap<String, WappalyzerRegex> = if let Some(js) = &technology_json.js {
                    js.iter()
                        .map(|(js_name, pattern)| (js_name.clone(), WappalyzerRegex::new(pattern)))
                        .collect()
                } else {
                    HashMap::new()
                };

                let script_src: Vec<WappalyzerRegex> = match &technology_json.script_src {
                    Some(StringUnion::String(pattern)) => vec![WappalyzerRegex::new(pattern)],
                    Some(StringUnion::Vec(patterns)) => patterns
                        .iter()
                        .map(|pattern| WappalyzerRegex::new(pattern))
                        .collect(),
                    None => vec![],
                };

                let meta: HashMap<String, WappalyzerRegex> = if let Some(meta) =
                    &technology_json.meta
                {
                    meta.iter()
                        .flat_map(|(meta_name, pattern)| match pattern {
                            StringUnion::String(pattern) => {
                                vec![(meta_name.clone(), WappalyzerRegex::new(pattern))]
                            }
                            StringUnion::Vec(patterns) => patterns
                                .iter()
                                .map(|pattern| (meta_name.clone(), WappalyzerRegex::new(pattern)))
                                .collect::<Vec<_>>(),
                        })
                        .collect()
                } else {
                    HashMap::new()
                };

                Technology {
                    name: name.clone(),
                    headers,
                    cookies,
                    html,
                    js,
                    script_src,
                    meta,
                }
            })
            .collect()
    }
}

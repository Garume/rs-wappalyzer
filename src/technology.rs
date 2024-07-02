use std::collections::HashMap;

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
    pub dns: Option<HashMap<String, Vec<String>>>,
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
    pub requires_category: Option<StringUnion>,
    pub meta: Option<HashMap<String, String>>,
    pub probe: Option<HashMap<String, String>>,
    #[serde(rename = "scriptSrc")]
    pub script_src: Option<StringUnion>,
    pub scripts: Option<StringUnion>,
    pub url: Option<String>,
    pub xhr: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum DomElement {
    Simple(String),
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
    pub(crate) technologies: Vec<Technology>,
}

impl TechnologyCollection {
    pub fn new() -> Self {
        TechnologyCollection {
            technologies: Vec::new(),
        }
    }
    pub fn from_json(json: &str) -> Self {
        let technology_jsons: HashMap<String, TechnologyJson> = serde_json::from_str(json).unwrap();
        let mut technologies = Vec::new();

        for (name, technology_json) in technology_jsons {
            let mut headers: HashMap<String, WappalyzerRegex> = HashMap::new();
            let mut cookies: HashMap<String, WappalyzerRegex> = HashMap::new();
            let mut html: Vec<WappalyzerRegex> = Vec::new();
            let mut js: HashMap<String, WappalyzerRegex> = HashMap::new();
            let mut script_src: Vec<WappalyzerRegex> = Vec::new();

            if let Some(technology_headers) = technology_json.headers {
                for (header, pattern) in technology_headers {
                    headers.insert(header, WappalyzerRegex::new(&pattern));
                }
            }

            if let Some(technology_cookies) = technology_json.cookies {
                for (cookie, pattern) in technology_cookies {
                    cookies.insert(cookie, WappalyzerRegex::new(&pattern));
                }
            }

            if let Some(technology_html) = technology_json.html {
                if let StringUnion::String(pattern) = technology_html {
                    html.push(WappalyzerRegex::new(&pattern));
                } else if let StringUnion::Vec(patterns) = technology_html {
                    for pattern in patterns {
                        html.push(WappalyzerRegex::new(&pattern));
                    }
                }
            }

            if let Some(technology_js) = technology_json.js {
                for (js_name, pattern) in technology_js {
                    js.insert(js_name, WappalyzerRegex::new(&pattern));
                }
            }

            if let Some(technology_script_src) = technology_json.script_src {
                if let StringUnion::String(pattern) = technology_script_src {
                    script_src.push(WappalyzerRegex::new(&pattern));
                } else if let StringUnion::Vec(patterns) = technology_script_src {
                    for pattern in patterns {
                        script_src.push(WappalyzerRegex::new(&pattern));
                    }
                }
            }

            technologies.push(Technology {
                name,
                headers,
                html,
                cookies,
                js,
                script_src,
            });
        }

        TechnologyCollection { technologies }
    }
}

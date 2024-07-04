use std::str::from_utf8;

use fancy_regex::{Captures, Regex};

#[derive(Debug)]
pub struct WappalyzerRegex {
    pub regex: Regex,
    pub confidence: u8,
    pub version_format: Option<String>,
}

impl WappalyzerRegex {
    pub fn new(pattern: &str) -> Self {
        let parts: Vec<&str> = pattern.split("\\;").collect();

        let regex = Regex::new(parts[0]);
        if regex.is_err() {
            if cfg!(debug_assertions) {
                eprintln!("Invalid regex pattern: {}", parts[0]);
            }

            return WappalyzerRegex {
                regex: Regex::new("").unwrap(),
                confidence: 0,
                version_format: None,
            };
        }

        let regex = regex.unwrap();

        let mut confidence = 100;
        let mut version_format = None;

        for part in parts.iter().skip(1) {
            if let Some(value) = part.strip_prefix("confidence:") {
                confidence = value.parse().unwrap_or(confidence);
            } else if let Some(value) = part.strip_prefix("version:") {
                version_format = Some(value.to_string());
            }
        }

        WappalyzerRegex {
            regex,
            confidence,
            version_format,
        }
    }

    pub fn extract(&self, input: &str) -> WappalyzerRegexResult {
        let captures = self.regex.captures(input);
        match captures {
            Ok(captures) => match captures {
                Some(captures) => {
                    let version = self.extract_version(captures);
                    WappalyzerRegexResult {
                        result: true,
                        version,
                        confidence: self.confidence,
                    }
                }
                None => WappalyzerRegexResult {
                    result: false,
                    version: "".to_string(),
                    confidence: 0,
                },
            },
            Err(_) => WappalyzerRegexResult {
                result: false,
                version: "".to_string(),
                confidence: 0,
            },
        }
    }

    pub fn extract_version(&self, captures: Captures) -> String {
        if let Some(version_format) = &self.version_format {
            let mut result = version_format.clone();

            for i in 1..=captures.len() {
                let group = captures
                    .get(i)
                    .map_or("", |m| from_utf8(m.as_str().as_bytes()).unwrap());
                result = result.replace(&format!("\\{}", i), group);
            }

            if let Some(index) = result.find('?') {
                let rest = &result[(index + 1)..];
                let (true_part, false_part) = match rest.find(':') {
                    Some(colon_index) => (&rest[..colon_index], &rest[(colon_index + 1)..]),
                    None => (rest, ""),
                };

                return if captures.get(1).is_some() {
                    true_part.to_string()
                } else {
                    false_part.to_string()
                };
            }

            return result;
        }
        "".to_string()
    }
}

impl From<&str> for WappalyzerRegex {
    fn from(pattern: &str) -> Self {
        WappalyzerRegex::new(pattern)
    }
}

pub struct WappalyzerRegexResult {
    pub result: bool,
    pub version: String,
    pub confidence: u8,
}

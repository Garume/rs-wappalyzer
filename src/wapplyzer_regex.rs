use std::str::from_utf8;

use regex::bytes::Regex;

#[derive(Debug)]
pub struct WappalyzerRegex {
    pub regex: Regex,
    pub confidence: u8,
    pub version_format: Option<String>,
}

impl WappalyzerRegex {
    pub fn new(pattern: &str) -> Self {
        let parts: Vec<&str> = pattern.split("\\;").collect();
        let regex = Regex::new(parts[0]).unwrap();

        let mut confidence = 100;
        let mut version_format = None;

        for part in parts.iter().skip(1) {
            if part.starts_with("confidence:") {
                confidence = part.split(":")
                    .collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap();
            } else if part.starts_with("version:") {
                version_format = Some(part.split_once(":")
                    .unwrap()
                    .1
                    .to_string());
            }
        }

        WappalyzerRegex {
            regex,
            confidence,
            version_format,
        }
    }

    pub fn extract_version(&self, input: &str) -> String {
        if let Some(version_format) = &self.version_format {
            if let Some(captures) = self.regex.captures(input.as_bytes()) {
                let mut result = version_format.clone();

                for i in 1..=captures.len() {
                    let group = captures.get(i).map_or("", |m| from_utf8(m.as_bytes()).unwrap());
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
        }
        "".to_string()
    }
}

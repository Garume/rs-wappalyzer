use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::wappalyzer::FingerPrintMeta;

pub struct HtmlChecker {
    // ...
}

impl Checker for HtmlChecker {
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let body = &page.body;
        let patterns = &technology.html;

        for pattern in patterns {
            let result = pattern.regex.is_match(body);

            match result {
                Ok(result) => {
                    if result {
                        return Some(FingerPrintMeta {
                            name: technology.name.clone(),
                            version: pattern.extract_version(body),
                            confidence: pattern.confidence as i32,
                        });
                    }
                }
                Err(e) => {
                    if cfg!(debug_assertions) {
                        eprintln!("Error matching regex: {}", e);
                        eprintln!("Regex: {:?}", pattern.regex);
                    }
                    continue;
                }
            }
        }

        None
    }
}

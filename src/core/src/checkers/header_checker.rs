use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::wappalyzer::FingerPrintMeta;

pub struct HeaderChecker {
    // ...
}

impl Checker for HeaderChecker {
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let headers = &page.headers;
        let patterns = &technology.headers;

        if headers.is_empty() {
            return None;
        }

        for pattern in patterns {
            if let Some(header) = headers.get(pattern.0) {
                let regex = pattern.1;
                let result = regex.regex.is_match(header.to_str().unwrap()).unwrap();
                if result {
                    return Some(FingerPrintMeta {
                        name: technology.name.clone(),
                        version: regex.extract_version(header.to_str().unwrap()),
                        confidence: regex.confidence as i32,
                    });
                }
            }
        }

        None
    }
}

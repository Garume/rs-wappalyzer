use crate::checkers::checker::Checker;
use crate::technology::Technology;
use crate::wappalyzer::FingerPrintMeta;
use crate::webpage::Webpage;

pub struct HeaderChecker {
    // ...
}

impl Checker for HeaderChecker {
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let headers = &page.headers;
        let patterns = &technology.headers;

        if headers.len() == 0 {
            return None;
        }

        for pattern in patterns {
            if let Some(header) = headers.get(pattern.0) {
                let regex = pattern.1;
                let result = regex.regex.is_match(header.as_ref());
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
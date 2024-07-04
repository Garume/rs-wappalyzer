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
                let result = regex.extract(header.to_str().unwrap());
                if result.result {
                    return Some(FingerPrintMeta {
                        name: technology.name.clone(),
                        version: result.version,
                        confidence: result.confidence as i32,
                    });
                }
            }
        }

        None
    }
}

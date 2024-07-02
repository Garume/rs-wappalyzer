use crate::checkers::checker::Checker;
use crate::technology::Technology;
use crate::wappalyzer::FingerPrintMeta;
use crate::webpage::Webpage;

pub struct HtmlChecker {
    // ...
}

impl Checker for HtmlChecker {
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let body = &page.body;
        let patterns = &technology.html;

        for pattern in patterns {
            let result = pattern.regex.is_match(body.as_bytes());
            if result {
                return Some(FingerPrintMeta {
                    name: technology.name.clone(),
                    version: pattern.extract_version(body),
                    confidence: pattern.confidence as i32,
                });
            }
        }

        None
    }
}

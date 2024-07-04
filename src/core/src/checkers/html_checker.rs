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
            let result = pattern.extract(body);

            if result.result {
                return Some(FingerPrintMeta {
                    name: technology.name.clone(),
                    version: result.version,
                    confidence: result.confidence as i32,
                });
            }
        }

        None
    }
}

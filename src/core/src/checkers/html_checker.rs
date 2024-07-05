use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::finger_print::FingerPrintMeta;

pub struct HtmlChecker {
    // ...
}

impl Checker for HtmlChecker {
    fn prepare(&mut self, _: &Webpage) {}

    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let body = &page.body;
        let patterns = &technology.html;

        for pattern in patterns {
            if let Some(result) = pattern.extract(body.as_ref()) {
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

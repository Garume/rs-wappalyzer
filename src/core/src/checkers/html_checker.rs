use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::checkers::checker::CheckResult;

pub struct HtmlChecker {
    // ...
}

impl Checker for HtmlChecker {
    fn prepare(&mut self, _: &Webpage) {}

    fn check(&self, page: &Webpage, technology: &Technology) -> Option<CheckResult> {
        let body = &page.body;
        let patterns = &technology.html;

        for pattern in patterns {
            if let Some(result) = pattern.extract(body.as_ref()) {
                return Some(CheckResult {
                    version: result.version,
                    confidence: result.confidence as i32,
                });
            }
        }

        None
    }
}

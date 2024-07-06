use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::checkers::checker::CheckResult;

pub struct HeaderChecker {
    // ...
}

impl Checker for HeaderChecker {
    fn prepare(&mut self, _: &Webpage) {}

    fn check(&self, page: &Webpage, technology: &Technology) -> Option<CheckResult> {
        let headers = &page.headers;
        let patterns = &technology.headers;

        if headers.is_empty() {
            return None;
        }

        for (key, regex) in patterns {
            if let Some(header) = headers.get(key) {
                if let Some(result) = regex.extract(header.to_str().unwrap()) {
                    return Some(CheckResult {
                        version: result.version,
                        confidence: result.confidence as i32,
                    });
                }
            }
        }

        None
    }
}

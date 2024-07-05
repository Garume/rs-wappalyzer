use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::finger_print::FingerPrintMeta;

pub struct HeaderChecker {
    // ...
}

impl Checker for HeaderChecker {
    fn prepare(&mut self, _: &Webpage) {}

    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let headers = &page.headers;
        let patterns = &technology.headers;

        if headers.is_empty() {
            return None;
        }

        for (key, regex) in patterns {
            if let Some(header) = headers.get(key) {
                if let Some(result) = regex.extract(header.to_str().unwrap()) {
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

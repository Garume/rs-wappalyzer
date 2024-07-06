use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::checkers::checker::CheckResult;

pub struct MetaChecker {
    // ...
}

impl Checker for MetaChecker {
    fn prepare(&mut self, _: &Webpage) {}

    fn check(&self, page: &Webpage, technology: &Technology) -> Option<CheckResult> {
        let nodes = &page.meta_nodes;
        let patterns = &technology.meta;

        for (pattern_key, regex) in patterns {
            if let Some(node) = nodes.get(pattern_key) {
                for value in node {
                    if let Some(result) = regex.extract(value.as_ref()) {
                        return Some(CheckResult {
                            version: result.version,
                            confidence: result.confidence as i32,
                        });
                    }
                }
            }
        }

        None
    }
}

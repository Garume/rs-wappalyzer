use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::checkers::checker::CheckResult;

pub struct ScriptSrcChecker {
    // ...
}

const SRC: &'static str = "src";

impl Checker for ScriptSrcChecker {
    fn prepare(&mut self, _: &Webpage) {}

    fn check(&self, page: &Webpage, technology: &Technology) -> Option<CheckResult> {
        let nodes = &page.script_nodes;
        let patterns = &technology.script_src;

        for pattern in patterns {
            if let Some(node) = nodes.get(SRC) {
                for value in node {
                    if let Some(result) = pattern.extract(value.as_ref()) {
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

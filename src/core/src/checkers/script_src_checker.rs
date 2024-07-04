use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::wappalyzer::FingerPrintMeta;

pub struct ScriptSrcChecker {
    // ...
}

impl Checker for ScriptSrcChecker {
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let nodes = &page.script_nodes;
        let patterns = &technology.script_src;

        for pattern in patterns {
            for node in nodes {
                let (key, values) = node;
                if key == "src" {
                    for value in values {
                        let result = pattern.extract(value.as_ref());
                        if result.result {
                            return Some(FingerPrintMeta {
                                name: technology.name.clone(),
                                version: result.version,
                                confidence: result.confidence as i32,
                            });
                        }
                    }
                }
            }
        }

        None
    }
}

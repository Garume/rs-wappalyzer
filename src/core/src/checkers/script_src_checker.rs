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
                        let result = pattern.regex.is_match(value).unwrap();
                        if result {
                            return Some(FingerPrintMeta {
                                name: technology.name.clone(),
                                version: pattern.extract_version(value),
                                confidence: pattern.confidence as i32,
                            });
                        }
                    }
                }
            }
        }

        None
    }
}

use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::wappalyzer::FingerPrintMeta;

pub struct MetaChecker {
    // ...
}

impl Checker for MetaChecker {
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let nodes = &page.meta_nodes;
        let patterns = &technology.meta;

        for pattern in patterns {
            for node in nodes {
                let (key, values) = node;
                if key == pattern.0 {
                    let regex = pattern.1;
                    for value in values {
                        let result = regex.regex.is_match(value).unwrap();
                        if result {
                            return Some(FingerPrintMeta {
                                name: technology.name.clone(),
                                version: pattern.1.extract_version(value),
                                confidence: pattern.1.confidence as i32,
                            });
                        }
                    }
                }
            }
        }

        None
    }
}

use markup5ever_rcdom::NodeData;

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
                if let NodeData::Element { attrs, .. } = &node {
                    for attr in attrs.borrow().iter() {
                        if attr.name.local.to_string() == "src" {
                            let result = pattern.regex.is_match(&attr.value.as_bytes());
                            if result {
                                return Some(FingerPrintMeta {
                                    name: technology.name.clone(),
                                    version: pattern.extract_version(&attr.value),
                                    confidence: pattern.confidence as i32,
                                });
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

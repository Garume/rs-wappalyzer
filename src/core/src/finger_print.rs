use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::technology::TechnologyCollection;

#[derive(Debug, Serialize, Deserialize)]
pub struct FingerPrint {
    pub data: HashSet<FingerPrintMeta>,
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FingerPrintMeta {
    pub name: String,
    pub version: String,
    pub confidence: i32,
    pub icon: Option<String>,
}

impl FingerPrint {
    pub fn new() -> Self {
        FingerPrint {
            data: HashSet::new(),
        }
    }
    pub fn analyze_implies(&mut self, technologies: &TechnologyCollection) {
        let mut entries = Vec::new();
        for finger_print in &self.data {
            if let Some(technology) = technologies.get(&finger_print.name) {
                for imply in &technology.implies {
                    entries.push(FingerPrintMeta {
                        name: imply.clone(),
                        version: "".to_string(),
                        confidence: 100,
                        icon: technology.icon.clone(),
                    });
                }
            }
        }

        for entry in entries {
            self.data.insert(entry);
        }
    }
}

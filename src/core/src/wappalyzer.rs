use rayon::prelude::*;

use crate::checker::{
    Checker, CookieChecker, HeaderChecker, HtmlChecker, MetaChecker, ScriptSrcChecker,
};
use crate::technology::TechnologyCollection;
use crate::webpage::Webpage;

pub struct Wappalyzer {
    technologies: TechnologyCollection,
    checkers: Vec<Box<dyn Checker + Send + Sync>>,
}

impl Wappalyzer {
    pub fn new() -> Self {
        Wappalyzer {
            technologies: TechnologyCollection::new(),
            checkers: vec![
                Box::new(CookieChecker {}),
                Box::new(HeaderChecker {}),
                Box::new(HtmlChecker {}),
                Box::new(MetaChecker {}),
                Box::new(ScriptSrcChecker {}),
            ],
        }
    }

    pub fn load_from_json(&mut self, json: &str) {
        let technologies = TechnologyCollection::from_json(json);
        self.technologies.add_collection(technologies);
    }

    pub fn analyze(&self, page: &Webpage) -> FingerPrint {
        let mut finger_print = FingerPrint { data: Vec::new() };

        for technology in &self.technologies.data {
            for checker in &self.checkers {
                let result = checker.check(page, technology);
                if let Some(meta) = result {
                    finger_print.data.push(meta);
                }
            }
        }

        finger_print
    }

    pub fn analyze_parallel(&self, page: &Webpage) -> FingerPrint {
        let finger_prints: Vec<FingerPrintMeta> = self
            .technologies
            .data
            .par_iter()
            .flat_map(|technology| {
                self.checkers
                    .par_iter()
                    .filter_map(|checker| checker.check(page, technology))
            })
            .collect();

        FingerPrint {
            data: finger_prints,
        }
    }
}

#[derive(Debug)]
pub struct FingerPrint {
    pub data: Vec<FingerPrintMeta>,
}

#[derive(Debug)]
pub struct FingerPrintMeta {
    pub name: String,
    pub version: String,
    pub confidence: i32,
}

#[cfg(test)]
mod tests {
    use tokio::fs;

    use super::*;

    #[tokio::test]
    async fn wappalyzer_latest_with_request() {
        let mut wappalyzer = Wappalyzer::new();
        let path = "tests/assets/technology_latest.json";
        let json = fs::read_to_string(path).await.unwrap();
        wappalyzer.load_from_json(json.as_str());
        let webpage = Webpage::from_url("http://example.com").await.unwrap();
        let result = wappalyzer.analyze(&webpage);
        println!("{:?}", result)
    }

    #[tokio::test]
    async fn wappalyzer_latest_with_request_parallel() {
        let mut wappalyzer = Wappalyzer::new();
        let path = "tests/assets/technology_latest.json";
        let json = fs::read_to_string(path).await.unwrap();
        wappalyzer.load_from_json(json.as_str());
        let webpage = Webpage::from_url("http://example.com").await.unwrap();
        let result = wappalyzer.analyze_parallel(&webpage);
        println!("{:?}", result)
    }
}

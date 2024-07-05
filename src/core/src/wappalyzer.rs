use std::collections::HashSet;

use rayon::prelude::*;

use crate::checker::{
    Checker, CookieChecker, HeaderChecker, HtmlChecker, MetaChecker, ScriptSrcChecker,
};
use crate::finger_print::{FingerPrint, FingerPrintMeta};
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
                Box::new(CookieChecker::new()),
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

    pub fn analyze(&mut self, page: &Webpage) -> FingerPrint {
        for checker in self.checkers.iter_mut() {
            checker.prepare(page);
        }

        let data: HashSet<FingerPrintMeta> = self
            .technologies
            .data
            .iter()
            .flat_map(|technology| {
                self.checkers
                    .iter()
                    .filter_map(|checker| checker.check(page, technology))
            })
            .collect();

        let mut finger_print = FingerPrint { data };
        finger_print.analyze_implies(&self.technologies);
        finger_print
    }

    pub fn analyze_parallel(&mut self, page: &Webpage) -> FingerPrint {
        for checker in self.checkers.iter_mut() {
            checker.prepare(page);
        }

        let data: HashSet<FingerPrintMeta> = self
            .technologies
            .data
            .par_iter()
            .flat_map(|technology| {
                self.checkers
                    .par_iter()
                    .filter_map(|checker| checker.check(page, technology))
            })
            .collect();

        let mut finger_print = FingerPrint { data };
        finger_print.analyze_implies(&self.technologies);
        finger_print
    }
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

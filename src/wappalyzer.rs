use crate::checkers::checker::Checker;
use crate::checkers::header_checker::HeaderChecker;
use crate::technology::TechnologyCollection;
use crate::webpage::Webpage;

pub struct Wappalyzer {
    technoloogies: TechnologyCollection,
    checkers: Vec<Box<dyn Checker>>,
}

impl Wappalyzer {
    pub fn new() -> Self {
        Wappalyzer {
            technoloogies: TechnologyCollection::new(),
            checkers: vec![Box::new(HeaderChecker {})],
        }
    }

    pub fn load(json: &str) -> Self {
        let mut wappalyzer = Wappalyzer::new();
        let technologies = TechnologyCollection::from_json(json);
        wappalyzer.technoloogies = technologies;
        wappalyzer
    }

    pub fn analyze(&self, page: &Webpage) -> FingerPrint {
        let mut finger_print = FingerPrint { data: Vec::new() };

        for technology in &self.technoloogies.technologies {
            for checker in &self.checkers {
                let result = checker.check(page, &technology);
                if let Some(meta) = result {
                    finger_print.data.push(meta);
                }
            }
        }

        finger_print
    }
}

#[derive(Debug)]
pub struct FingerPrint {
    data: Vec<FingerPrintMeta>,
}

#[derive(Debug)]
pub struct FingerPrintMeta {
    pub name: String,
    pub version: String,
    pub confidence: i32,
}

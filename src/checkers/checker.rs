use crate::technology::Technology;
use crate::wappalyzer::FingerPrintMeta;
use crate::webpage::Webpage;

pub trait Checker {
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta>;
}
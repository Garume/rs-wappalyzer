use crate::technology::Technology;
use crate::webpage::Webpage;

pub trait Checker: Send + Sync {
    fn prepare(&mut self, page: &Webpage);
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<CheckResult>;
}

pub struct CheckResult {
    pub(crate) version: String,
    pub(crate) confidence: i32,
}

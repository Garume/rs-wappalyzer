use crate::finger_print::FingerPrintMeta;
use crate::technology::Technology;
use crate::webpage::Webpage;

pub trait Checker: Send + Sync {
    fn prepare(&mut self, page: &Webpage);
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta>;
}

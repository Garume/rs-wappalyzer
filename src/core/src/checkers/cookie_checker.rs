use std::collections::HashMap;

use reqwest::header::SET_COOKIE;

use crate::checkers::checker::Checker;
use crate::finger_print::FingerPrintMeta;
use crate::technology::Technology;
use crate::webpage::Webpage;

pub struct CookieChecker {
    cookies: HashMap<String, String>,
    already_prepared: bool,
}

impl CookieChecker {
    pub fn new() -> Self {
        CookieChecker {
            cookies: HashMap::new(),
            already_prepared: false,
        }
    }
}

impl Checker for CookieChecker {
    fn prepare(&mut self, page: &Webpage) {
        if let Some(cookie_header) = page.headers.get(SET_COOKIE) {
            for cookie in cookie_header.to_str().unwrap().split("; ") {
                let parts: Vec<&str> = cookie.splitn(2, '=').collect();
                if parts.len() == 2 {
                    self.cookies
                        .insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }

        self.already_prepared = true;
    }

    fn check(&self, _: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        if !self.already_prepared {
            panic!("Checker not prepared");
        }

        let patterns = &technology.cookies;

        for (key, regex) in patterns {
            if let Some(cookie) = self.cookies.get(key) {
                if let Some(result) = regex.extract(cookie.as_ref()) {
                    return Some(FingerPrintMeta {
                        name: technology.name.clone(),
                        version: result.version,
                        confidence: result.confidence as i32,
                    });
                }
            }
        }

        None
    }
}

use std::collections::HashMap;

use reqwest::header::SET_COOKIE;

use crate::checkers::checker::Checker;
use crate::technology::Technology;
use crate::wappalyzer::FingerPrintMeta;
use crate::webpage::Webpage;

pub struct CookieChecker {
    // ...
}

impl Checker for CookieChecker {
    fn check(&self, page: &Webpage, technology: &Technology) -> Option<FingerPrintMeta> {
        let patterns = &technology.cookies;
        let mut cookies: HashMap<String, String> = HashMap::new();

        if let Some(cookie_header) = page.headers.get(SET_COOKIE) {
            for cookie in cookie_header.to_str().unwrap().split("; ") {
                let parts: Vec<&str> = cookie.splitn(2, '=').collect();
                if parts.len() == 2 {
                    cookies.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }

        for pattern in patterns {
            if let Some(cookie) = cookies.get(pattern.0) {
                let regex = pattern.1;
                let result = regex.regex.is_match(cookie.as_ref()).unwrap();
                if result {
                    return Some(FingerPrintMeta {
                        name: technology.name.clone(),
                        version: regex.extract_version(cookie),
                        confidence: regex.confidence as i32,
                    });
                }
            }
        }

        None
    }
}

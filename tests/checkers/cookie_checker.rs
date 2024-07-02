use std::collections::HashMap;

use rs_wappalyzer::{Technology, WappalyzerRegex, Webpage};
use rs_wappalyzer::checker::{Checker, CookieChecker};

#[test]
fn cookie_checker_works() {
    let checker = CookieChecker {};
    let webpage = Webpage::default().with_headers(HashMap::from([("Set-Cookie", "cookie=value")]));
    let mut technology = Technology::default("Example");
    technology
        .cookies
        .insert("cookie".to_string(), WappalyzerRegex::new("value"));
    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn cookie_checker_works_with_regex() {
    let checker = CookieChecker {};
    let webpage = Webpage::default().with_headers(HashMap::from([("Set-Cookie", "apikey=hello")]));
    let mut technology = Technology::default("Example");
    technology
        .cookies
        .insert("apikey".to_string(), WappalyzerRegex::new("^[\\w\\d-]+$"));
    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

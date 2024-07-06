use std::collections::HashMap;

use wappalyzer_core::{Technology, Webpage};
use wappalyzer_core::checker::{Checker, HeaderChecker};

#[test]
fn checker_works() {
    let checker = HeaderChecker {};
    let webpage = Webpage::default().with_headers(HashMap::from([("Content-Type", "text/html")]));
    let mut technology = Technology::default("Example");
    technology
        .headers
        .insert("Content-Type".to_string(), "text/html".into());
    let result = checker.check(&webpage, &technology);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn checker_works_with_regex() {
    let checker = HeaderChecker {};
    let webpage = Webpage::default().with_headers(HashMap::from([("Content-Type", "text/html")]));
    let mut technology = Technology::default("Example");
    technology
        .headers
        .insert("Content-Type".to_string(), "text/.*".into());
    let result = checker.check(&webpage, &technology);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn checker_works_with_regex_and_version() {
    let checker = HeaderChecker {};
    let webpage = Webpage::default().with_headers(HashMap::from([("server", "server/1.0.0")]));
    let mut technology = Technology::default("Example");
    technology.headers.insert(
        "server".to_string(),
        "server/?([\\d.]+)?\\;version:\\1".into(),
    );
    let result = checker.check(&webpage, &technology).unwrap();
    assert_eq!(result.name, "Example".to_string());
    assert_eq!(result.version, "1.0.0".to_string());
}

#[test]
fn checker_works_with_regex_fail() {
    let checker = HeaderChecker {};
    let webpage = Webpage::default().with_headers(HashMap::from([("Content-Type", "text/html")]));
    let mut technology = Technology::default("Example");
    technology
        .headers
        .insert("Content-Type".to_string(), "text/css".into());
    let result = checker.check(&webpage, &technology);
    assert_eq!(result.is_none(), true);
}

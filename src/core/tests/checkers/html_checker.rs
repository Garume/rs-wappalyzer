use core::{Technology, Webpage};
use core::checker::{Checker, HtmlChecker};

#[test]
fn checker_works() {
    let checker = HtmlChecker {};
    let webpage = Webpage::default().with_body("<html>Example</html>");
    let mut technology = Technology::default("Example");
    technology.html.push("Example".into());
    let result = checker.check(&webpage, &technology);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn checker_works_with_regex() {
    let checker = HtmlChecker {};
    let webpage = Webpage::default().with_body("<html>Example</html>");
    let mut technology = Technology::default("Example");
    technology.html.push("Ex.*".into());
    let result = checker.check(&webpage, &technology);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn checker_works_with_regex_and_version() {
    let checker = HtmlChecker {};
    let webpage = Webpage::default().with_body("<html>Example 1.0.0</html>");
    let mut technology = Technology::default("Example");
    technology
        .html
        .push("Example ?([\\d.]+)?\\;version:\\1".into());
    let result = checker.check(&webpage, &technology).unwrap();
    assert_eq!(result.name, "Example".to_string());
    assert_eq!(result.version, "1.0.0".to_string());
}

#[test]
fn checker_works_with_regex_fail() {
    let checker = HtmlChecker {};
    let webpage = Webpage::default().with_body("<html>example</html>");
    let mut technology = Technology::default("Example");
    technology.html.push("Ex.*".into());
    let result = checker.check(&webpage, &technology);
    assert_eq!(result.is_none(), true);
}

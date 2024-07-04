use std::collections::HashMap;

use core::{Technology, WappalyzerRegex, Webpage};
use core::checker::{Checker, ScriptSrcChecker};

fn create_script_node(src: &str) -> HashMap<String, Vec<String>> {
    let mut node = HashMap::new();
    node.insert("src".to_string(), vec![src.to_string()]);
    node
}

#[test]
fn checker_works() {
    let checker = ScriptSrcChecker {};
    let webpage = Webpage::default().with_script_nodes(create_script_node("test.js"));
    let mut technology = Technology::default("Example");
    technology
        .script_src
        .push(WappalyzerRegex::new("test\\.js"));

    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn checker_works_with_regex() {
    let checker = ScriptSrcChecker {};
    let webpage = Webpage::default().with_script_nodes(create_script_node("test.js"));
    let mut technology = Technology::default("Example");
    technology
        .script_src
        .push(WappalyzerRegex::new("test\\..*"));

    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn checker_works_with_regex_and_version() {
    let checker = ScriptSrcChecker {};
    let webpage = Webpage::default().with_script_nodes(create_script_node("test-1.0.0.js"));
    let mut technology = Technology::default("Example");
    technology
        .script_src
        .push(WappalyzerRegex::new("test-([\\d.]+)?\\.js\\;version:\\1"));

    let result = checker.check(&webpage, &technology).unwrap();

    assert_eq!(result.name, "Example".to_string());
    assert_eq!(result.version, "1.0.0".to_string());
}

#[test]
fn checker_works_with_regex_fail() {
    let checker = ScriptSrcChecker {};
    let webpage = Webpage::default().with_script_nodes(create_script_node("test.js"));
    let mut technology = Technology::default("Example");
    technology
        .script_src
        .push(WappalyzerRegex::new("test\\.css"));

    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_none(), true);
}

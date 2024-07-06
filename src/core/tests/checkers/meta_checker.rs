use std::collections::HashMap;

use wappalyzer_core::{Technology, Webpage};
use wappalyzer_core::checker::{Checker, MetaChecker};

fn create_meta_node(name: &str, content: &str) -> HashMap<String, Vec<String>> {
    let mut node = HashMap::new();
    node.insert(name.to_string(), vec![content.to_string()]);
    node
}

#[test]
fn meta_checker_works() {
    let checker = MetaChecker {};
    let webpage = Webpage::default().with_meta_nodes(create_meta_node("generator", "Example"));
    let mut technology = Technology::default("Example");
    technology
        .meta
        .insert("generator".to_string(), "Example".into());
    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn meta_checker_works_with_regex() {
    let checker = MetaChecker {};
    let webpage = Webpage::default().with_meta_nodes(create_meta_node("generator", "Example"));
    let mut technology = Technology::default("Example");
    technology
        .meta
        .insert("generator".to_string(), "Ex.*".into());
    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

#[test]
fn meta_checker_works_with_regex_and_version() {
    let checker = MetaChecker {};
    let webpage =
        Webpage::default().with_meta_nodes(create_meta_node("generator", "Example 1.0.0"));
    let mut technology = Technology::default("Example");
    technology.meta.insert(
        "generator".to_string(),
        "Example ?([\\d.]+)?\\;version:\\1".into(),
    );
    let result = checker.check(&webpage, &technology).unwrap();

    assert_eq!(result.name, "Example".to_string());
    assert_eq!(result.version, "1.0.0".to_string());
}

#[test]
fn meta_checker_works_with_regex_fail() {
    let checker = MetaChecker {};
    let webpage = Webpage::default().with_meta_nodes(create_meta_node("generator", "example"));
    let mut technology = Technology::default("Example");
    technology
        .meta
        .insert("generator".to_string(), "Ex.*".into());
    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_none(), true);
}

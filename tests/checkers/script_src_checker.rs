use std::cell::RefCell;

use html5ever::{Attribute, LocalName, Namespace, QualName};
use markup5ever_rcdom::NodeData;

use rs_wappalyzer::{Technology, WappalyzerRegex, Webpage};
use rs_wappalyzer::checker::{Checker, ScriptSrcChecker};

#[test]
fn checker_works() {
    let checker = ScriptSrcChecker {};
    let webpage = Webpage::default().with_script_nodes(vec![NodeData::Element {
        name: QualName::new(None, Namespace::from(""), LocalName::from("script")),
        attrs: RefCell::new(vec![Attribute {
            name: QualName::new(None, Namespace::from(""), LocalName::from("src")),
            value: "test.js".into(),
        }]),
        template_contents: RefCell::new(None),
        mathml_annotation_xml_integration_point: false,
    }]);
    let mut technology = Technology::default("Example");
    technology
        .script_src
        .push(WappalyzerRegex::new("test\\.js"));

    let result = checker.check(&webpage, &technology);

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().name, "Example".to_string());
}

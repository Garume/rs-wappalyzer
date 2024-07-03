use std::collections::HashMap;

use rs_wappalyzer::wappalyzer::Wappalyzer;
use rs_wappalyzer::webpage::Webpage;

#[test]
fn wappalyzer_header() {
    let path = "tests/assets/technology_simple.json";
    let json = std::fs::read_to_string(path).unwrap();
    let wappalyzer = Wappalyzer::load(json.as_str());
    let webpage = Webpage::default().with_headers(HashMap::from([("X-Powered-CMS", "Bitrix Site Manager/1.0.0")]));
    let result = wappalyzer.analyze(&webpage);
    println!("{:?}", result)
}

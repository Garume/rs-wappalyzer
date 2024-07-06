use std::collections::HashMap;

use wappalyzer_core::{Wappalyzer, Webpage};

#[test]
fn wappalyzer_latest() {
    let mut wappalyzer = Wappalyzer::new();
    let path = "tests/assets/technology_latest.json";
    let json = std::fs::read_to_string(path).unwrap();
    wappalyzer.load_from_json(json.as_str());

    let webpage = Webpage::default().with_headers(HashMap::from([("Server", "AOLserver/1.0.0")]));
    let result = wappalyzer.analyze(&webpage);
    println!("{:?}", result)
}

/*#[tokio::test]
async fn wappalyzer_latest_with_request() {
    let path = "tests/assets/technology_latest.json";
    let json = fs::read_to_string(path).await.unwrap();
    let wappalyzer = Wappalyzer::load(json.as_str());
    let webpage = Webpage::from_url("http://example.com").await.unwrap();
    let result = wappalyzer.analyze(&webpage);
    println!("{:?}", result)
}*/

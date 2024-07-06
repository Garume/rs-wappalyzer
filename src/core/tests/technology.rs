use std::collections::HashMap;

use wappalyzer_core::technology::{DomComplex, DomElement, TechnologyJson};

#[test]
fn deserialize_example_technology() {
    let file_path = "tests/assets/technology_test.json";
    let data = std::fs::read_to_string(file_path).unwrap();

    let json_data: HashMap<String, TechnologyJson> = serde_json::from_str(&data).unwrap();
    let technology = json_data.get("Example").unwrap();

    assert_eq!(
        technology.description,
        Some("A short description of the technology.".to_string())
    );
    assert_eq!(technology.cats, vec![1]);
    assert_eq!(
        technology.cookies.as_ref().unwrap().get("cookie_name"),
        Some(&"Example".to_string())
    );

    if let Some(DomElement::Nested(nested)) = &technology.dom {
        if let Some(DomElement::Complex(DomComplex {
            exists,
            attributes,
            properties,
            text,
        })) = nested.get("#example-id")
        {
            assert_eq!(exists, "");
            assert_eq!(attributes.get("class"), Some(&"example-class".to_string()));
            assert_eq!(properties.get("example-property"), Some(&"".to_string()));
            assert_eq!(text, "Example text content");
        }
    }

    assert_eq!(
        technology.dns.as_ref().unwrap().get("MX").unwrap(),
        &vec!["example\\.com"].into()
    );
    assert_eq!(
        technology.js.as_ref().unwrap().get("Example.method"),
        Some(&"".to_string())
    );
    assert_eq!(technology.excludes, Some("Example".into()));
    assert_eq!(
        technology.headers.as_ref().unwrap().get("X-Powered-By"),
        Some(&"Example".to_string())
    );
    assert_eq!(technology.html, Some("<link[^>]example\\.css".into()));
    assert_eq!(technology.text, Some("\u{8}example\u{8}".into()));
    assert_eq!(technology.css, Some("\\.example-class".into()));
    assert_eq!(technology.robots, Some("Disallow: /unique-path/".into()));
    assert_eq!(technology.implies, Some("PHP\\;confidence:50".into()));
    assert_eq!(technology.requires, Some("WordPress".into()));
    assert_eq!(technology.requires_category, Some("Ecommerce".into()));
    assert_eq!(
        technology.meta.as_ref().unwrap().get("generator"),
        Some(&"(?:Example|Another Example)".into())
    );
    assert_eq!(
        technology.probe.as_ref().unwrap().get("/path"),
        Some(&"".to_string())
    );
    assert_eq!(
        technology.script_src,
        Some("example-([0-9.]+)\\.js\\;confidence:50\\;version:\\1".into())
    );
    assert_eq!(
        technology.scripts,
        Some("function webpackJsonpCallback\\(data\\) {".into())
    );
    assert_eq!(technology.url, Some("example\\.com".into()));
    assert_eq!(technology.xhr, Some("example\\.com".into()));
    assert_eq!(technology.oss, Some(true));
    assert_eq!(technology.saas, Some(true));
    assert_eq!(
        technology.pricing,
        Some(vec![
            "mid".to_string(),
            "freemium".to_string(),
            "recurring".to_string()
        ])
    );
    assert_eq!(technology.website, "https://example.com".to_string());
}

#[test]
fn deserialize_simple_technology() {
    let file_path = "tests/assets/technology_simple.json";
    let data = std::fs::read_to_string(file_path).unwrap();

    let json_data: HashMap<String, TechnologyJson> = serde_json::from_str(&data).unwrap();

    // Test 1C-Bitrix
    if let Some(bitrix) = json_data.get("1C-Bitrix") {
        print!("{:?}", bitrix);

        assert_eq!(bitrix.cats, vec![1, 6]);
        assert_eq!(
            bitrix.cookies.as_ref().unwrap().get("BITRIX_SM_GUEST_ID"),
            Some(&"".to_string())
        );
        assert_eq!(bitrix.description, Some("1C-Bitrix is a system of web project management, universal software for the creation, support and successful development of corporate websites and online stores.".to_string()));
        assert_eq!(
            bitrix.headers.as_ref().unwrap().get("Set-Cookie"),
            Some(&"BITRIX_".to_string())
        );
        assert_eq!(bitrix.icon, Some("1C-Bitrix.svg".into()));
        assert_eq!(bitrix.implies, Some("PHP".into()));
        assert_eq!(
            bitrix.pricing,
            Some(vec![
                "onetime".to_string(),
                "mid".to_string(),
                "recurring".to_string()
            ])
        );
        assert_eq!(bitrix.saas, Some(true));
        assert_eq!(
            bitrix.script_src,
            Some("bitrix(?:\\.info/|/js/main/core)".into())
        );
        assert_eq!(bitrix.website, "https://www.1c-bitrix.ru".to_string());
    }

    // Test 2B Advice
    if let Some(advice) = json_data.get("2B Advice") {
        assert_eq!(advice.cats, vec![67]);
        assert_eq!(
            advice.description,
            Some("2B Advice provides a plug-in to manage GDPR cookie consent.".to_string())
        );
        assert_eq!(advice.icon, Some("2badvice.png".to_string()));
        assert_eq!(
            advice.js.as_ref().unwrap().get("BBCookieControler"),
            Some(&"".to_string())
        );
        assert_eq!(advice.saas, Some(true));
        assert_eq!(
            advice.script_src,
            Some("2badvice-cdn\\.azureedge\\.net".into())
        );
        assert_eq!(
            advice.website,
            "https://www.2b-advice.com/en/data-privacy-software/cookie-consent-plugin/".to_string()
        );
    }

    // Test 30namaPlayer
    if let Some(player) = json_data.get("30namaPlayer") {
        assert_eq!(player.cats, vec![14]);
        assert_eq!(player.description, Some("30namaPlayer is a modified version of Video.js to work with videos on HTML using javascript.".to_string()));
        assert_eq!(
            player.dom,
            Some(DomElement::Simple(
                "section[class*='player30nama']".to_string()
            ))
        );
        assert_eq!(player.icon, Some("30namaPlayer.png".to_string()));
        assert_eq!(player.website, "https://30nama.com/".to_string());
    }
}

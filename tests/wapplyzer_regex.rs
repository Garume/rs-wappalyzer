use rs_wappalyzer::wapplyzer_regex::WappalyzerRegex;

#[test]
fn wapplyzer_regex_create() {
    struct Test<'a> {
        input: &'a str,
        expected_regex: &'a str,
        expected_confidence: u8,
        expected_version_format: Option<String>,
    }

    let tests = vec![
        Test {
            input: "Example.*",
            expected_regex: "Example.*",
            expected_confidence: 100,
            expected_version_format: None,
        },
        Test {
            input: "Example.*\\;confidence:50",
            expected_regex: "Example.*",
            expected_confidence: 50,
            expected_version_format: None,
        },
        Test {
            input: "Example-([0-9.]+)\\.js\\;version:\\1",
            expected_regex: "Example-([0-9.]+)\\.js",
            expected_confidence: 100,
            expected_version_format: Some("\\1".to_string()),
        },
    ];

    for test in tests {
        let regex = WappalyzerRegex::new(test.input);
        assert_eq!(regex.regex.as_str(), test.expected_regex);
        assert_eq!(regex.confidence, test.expected_confidence);
        assert_eq!(regex.version_format, test.expected_version_format);
    }
}

#[test]
fn wapplyzer_regex_extract_version() {
    struct Test<'a> {
        pattern: &'a str,
        input: &'a str,
        expected_version: &'a str,
    }

    let tests = vec![
        Test {
            pattern: "Example ([0-9.]+)\\;version:\\1",
            input: "Example 1.0.0",
            expected_version: "1.0.0",
        },
        Test {
            pattern: "Example ([0-9.]+)\\;version:\\1?found:",
            input: "Example 1.0.0",
            expected_version: "found",
        },
        Test {
            pattern: "Example\\;version:\\1?:not found",
            input: "Example",
            expected_version: "not found",
        },
        Test {
            pattern: "([\\d.]+)?/Example(?:\\.([\\d.]+))?.*\\.js\\;version:\\1?\\1:\\2",
            input: "2.6.2/Example.js",
            expected_version: "2.6.2",
        },
        Test {
            pattern: "([\\d.]+)?/Example(?:\\.([\\d.]+))?.*\\.js\\;version:\\1?\\1:\\2",
            input: "/Example.2.6.2.js",
            expected_version: "2.6.2",
        },
        Test {
            pattern: "(?:Example(?:$|/([\\d.]+)|[^/-])|(?:^|\\b)httpd)\\;version:\\1",
            input: "Example",
            expected_version: "",
        },
        Test {
            pattern: "(?:Example(?:$|/([\\d.]+)|[^/-])|(?:^|\\b)httpd)\\;version:\\1",
            input: "Example/1.0.0",
            expected_version: "1.0.0",
        },
    ];

    for test in tests {
        let regex = WappalyzerRegex::new(test.pattern);
        let version = regex.extract_version(test.input);
        assert_eq!(version, test.expected_version);
    }
}
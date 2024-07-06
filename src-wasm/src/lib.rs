use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use wappalyzer_core::FingerPrint;

#[wasm_bindgen]
pub async fn web_analyze(url: &str, json_data: JsValue) -> Result<JsValue, JsValue> {
    let json_data: Vec<String> = json_data
        .into_serde()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let result = analyze(url, json_data)
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_serde(&result).unwrap())
}

async fn analyze(url: &str, json_data: Vec<String>) -> Result<FingerPrint, AnalyzeError> {
    let mut wappalyzer = wappalyzer_core::Wappalyzer::new();

    for json in json_data {
        wappalyzer.load_from_json(&json);
    }

    let webpage = wappalyzer_core::Webpage::from_url(url)
        .await
        .map_err(|_| AnalyzeError::AnalyzeError())?;
    let result = wappalyzer.analyze(&webpage);
    Ok(result)
}

#[derive(Debug, thiserror::Error)]
enum AnalyzeError {
    #[error("failed to read directory")]
    ReadDirError(),
    #[error("failed to read file")]
    ReadFileError(),
    #[error("failed to analyze webpage")]
    AnalyzeError(),
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::generate_context;

use wappalyzer_core::FingerPrint;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![web_analyze])
        .run(generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
enum AnalyzeError {
    #[error("failed to read directory")]
    ReadDirError(),
    #[error("failed to read file")]
    ReadFileError(),
    #[error("failed to analyze webpage")]
    AnalyzeError(),
}

#[tauri::command]
async fn web_analyze(url: String) -> Result<FingerPrint, AnalyzeError> {
    let mut wappalyzer = wappalyzer_core::Wappalyzer::new();
    let folder_path = "C:/Users/user/RustroverProjects/rs-wappalyzer/src/core/benches/latest";
    let files = std::fs::read_dir(folder_path).map_err(|_| AnalyzeError::ReadDirError())?;

    for file in files {
        let file = file.map_err(|_| AnalyzeError::ReadDirError())?;
        let path = file.path();
        let json = std::fs::read_to_string(path).map_err(|_| AnalyzeError::ReadFileError())?;
        wappalyzer.load_from_json(json.as_str());
    }

    let webpage = wappalyzer_core::Webpage::from_url(url.as_str()).await;

    match webpage {
        Ok(webpage) => {
            let result = wappalyzer.analyze(&webpage);
            Ok(result)
        }
        Err(_) => Err(AnalyzeError::AnalyzeError()),
    }
}

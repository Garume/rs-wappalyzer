use std::fs;

use criterion::*;
use tokio::runtime::Runtime;

use core::{FingerPrint, Wappalyzer, Webpage};

fn load_technologies() -> Wappalyzer {
    let mut wappalyzer = Wappalyzer::new();

    let files =
        fs::read_dir("C:/Users/user/RustroverProjects/rs-wappalyzer/src/core/benches/latest")
            .unwrap();
    for file in files {
        let path = file.unwrap().path();
        let json = fs::read_to_string(path).unwrap();
        wappalyzer.load_from_json(json.as_str());
    }

    wappalyzer
}

fn deserialize_technologies() {
    let files =
        fs::read_dir("C:/Users/user/RustroverProjects/rs-wappalyzer/src/core/benches/latest")
            .unwrap();
    for file in files {
        let path = file.unwrap().path();
        let json = fs::read_to_string(path).unwrap();
        let _ = serde_json::from_str::<serde_json::Value>(json.as_str());
    }
}

async fn analyze_request(url: &str) -> FingerPrint {
    let mut wappalyzer = Wappalyzer::new();

    let files =
        fs::read_dir("C:/Users/user/RustroverProjects/rs-wappalyzer/src/core/benches/latest")
            .unwrap();
    for file in files {
        let path = file.unwrap().path();
        let json = fs::read_to_string(path).unwrap();
        wappalyzer.load_from_json(json.as_str());
    }

    let webpage = Webpage::from_url(url).await.unwrap();

    wappalyzer.analyze(&webpage)
}

fn bench_deserialize_technologies(c: &mut Criterion) {
    c.bench_function("deserialize_technologies", |b| {
        b.iter(|| {
            deserialize_technologies();
        })
    });
}

fn bench_load_technologies(c: &mut Criterion) {
    c.bench_function("load_technologies", |b| {
        b.iter(|| {
            load_technologies();
        })
    });
}

fn bench_wappalyzer(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("wappalyzer_latest", move |b| {
        b.iter(|| {
            rt.block_on(async {
                let _ = analyze_request("http://example.com").await;
            });
        })
    });
}

fn bench_only_analyze(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("only_analyze", move |b| {
        b.iter(|| {
            let mut wappalyzer = Wappalyzer::new();
            let files = fs::read_dir(
                "C:/Users/user/RustroverProjects/rs-wappalyzer/src/core/benches/latest",
            )
            .unwrap();
            for file in files {
                let path = file.unwrap().path();
                let json = fs::read_to_string(path).unwrap();
                wappalyzer.load_from_json(json.as_str());
            }

            rt.block_on(async {
                let webpage = Webpage::from_url("http://example.com").await.unwrap();
                let _ = wappalyzer.analyze(&webpage);
            });
        })
    });
}

criterion_group!(
    benches,
    bench_deserialize_technologies,
    bench_load_technologies,
    bench_only_analyze,
    bench_wappalyzer
);

// main関数を用意
criterion_main!(benches);

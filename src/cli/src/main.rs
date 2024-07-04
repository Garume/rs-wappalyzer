use std::fs;

use clap::Parser;

use core::{FingerPrint, Wappalyzer, Webpage};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL to analyze
    #[arg(short, long)]
    url: String,

    // Path to the technology json files
    #[arg(short, long)]
    path: String,

    /// Whether to parallelize the analysis
    #[arg(long)]
    parallel: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let time = std::time::Instant::now();

    let fingerprint = if args.parallel {
        analyze_parallel_request(args.url.as_str(), args.path.as_str()).await
    } else {
        analyze_request(args.url.as_str(), args.path.as_str()).await
    };

    for meta in fingerprint.data {
        println!("{},{}", meta.name, meta.version);
    }

    println!("Elapsed time: {:?}", time.elapsed());
}

async fn analyze_request(url: &str, path: &str) -> FingerPrint {
    let mut wappalyzer = Wappalyzer::new();

    let files = fs::read_dir(path).unwrap();
    for file in files {
        let path = file.unwrap().path();
        let json = fs::read_to_string(path).unwrap();
        wappalyzer.load_from_json(json.as_str());
    }

    let webpage = Webpage::from_url(url).await.unwrap();

    wappalyzer.analyze(&webpage)
}

async fn analyze_parallel_request(url: &str, path: &str) -> FingerPrint {
    let mut wappalyzer = Wappalyzer::new();

    let files = fs::read_dir(path).unwrap();
    for file in files {
        let path = file.unwrap().path();
        let json = fs::read_to_string(path).unwrap();
        wappalyzer.load_from_json(json.as_str());
    }

    let webpage = Webpage::from_url(url).await.unwrap();

    wappalyzer.analyze_parallel(&webpage)
}

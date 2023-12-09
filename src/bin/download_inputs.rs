use bytes::Bytes;
use chrono::{Datelike, Utc};
use clap::Parser;
use std::{env::var, path::PathBuf};

#[derive(Debug, Clone, Parser)]
pub struct Opts {
    #[arg(env = "AOC_SESSION")]
    aoc_session: Option<String>,
    #[arg(default_value = None)]
    day: Option<usize>,
    #[arg(default_value = None)]
    data_dir: Option<PathBuf>,
}

pub async fn download_inputs(
    day: u32,
    client: &reqwest::Client,
    session: &str,
) -> Result<bytes::Bytes, String> {
    let url = format!("https://adventofcode.com/2023/day/{day}/input");

    let Ok(resp) = 
    client
    .get(&url)
    .header("Cookie", format!("session={session}"))
    .send()
    .await else {
        return Err(
            format!("failed to download input for day: {}", day)
        );
    };

    if resp.status() != 200 {
        return Err(format!(
            "could not get a successful response for day: {}",
            day
        ));
    }

    resp.bytes().await.map_err(|err| err.to_string())
}

pub async fn save_inputs<P>(day: u32, contents: Bytes, root_dir: P) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    let root_dir = root_dir.as_ref();
    let filepath = root_dir.join(format!("{:02}.in", day));
    let res = tokio::fs::write(filepath.clone(), contents).await;
    if res.is_ok() {
        eprintln!("Downloaded inputs for day {day:02} to: {}", filepath.to_string_lossy());
    }
    res
}

pub async fn extract_input<P>(
    date: u32,
    client: reqwest::Client,
    root_dir: P,
    session: String,
) -> std::result::Result<(), String>
where
    P: AsRef<std::path::Path>,
{
    let contents = download_inputs(date, &client, &session).await?;
    save_inputs(date, contents, root_dir)
        .await
        .map_err(|err| err.to_string())
}

pub async fn extract_inputs<P>(
    day_range: std::ops::Range<u32>,
    client: &reqwest::Client,
    root_dir: P,
    session: String,
) where
    P: AsRef<std::path::Path> + Clone + Send + Sync + 'static,
{
    let mut handles = vec![];
    for date in day_range {
        let client = client.clone();
        let root_dir = root_dir.clone();
        let session = session.clone();

        handles.push(tokio::task::spawn(async move {
            if let Err(err) = extract_input(date, client, root_dir, session.clone()).await {
                eprintln!("{}", err);
            }
        }));
    }

    for handle in handles {
        _ = handle.await;
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().unwrap();
    let opts = Opts::parse();

    let date_range = match opts.day {
        Some(day) => day as u32..day as u32 + 1,
        None => 1..Utc::now().day() + 1,
    };

    let client = reqwest::Client::new();
    let root_dir = match opts.data_dir {
        Some(dir) => dir,
        None => {
            let path =
                std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("data/");
            println!("{:#?}", path);
            path
        }
    };
    let aoc_session = match opts.aoc_session {
        None => var("AOC_SESSION").expect("No AOC_SESSION provided."),
        Some(session) => session,
    };

    extract_inputs(date_range, &client, root_dir, aoc_session).await;

    Ok(())
}

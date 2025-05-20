use std::path::PathBuf;

use clap::Parser;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, COOKIE, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    cookies: String,
    room_id: String,
    csrf: String,
    csrf_token: String,
}

#[derive(Debug, clap::Parser)]
struct Cli {
    #[clap(short, long, default_value = "config.toml")]
    path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let context = tokio::fs::read_to_string(&cli.path).await?;
    let config: Config = toml::from_str(&context)?;

    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    header.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    header.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
    );
    header.insert(COOKIE, HeaderValue::from_str(&config.cookies)?);
    header.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36 Edg/129.0.0.0"));

    let body = format!(
        "room_id={}&csrf={}&csrf_token={}&platform=pc&area_v2=624&backup_stream=0",
        config.room_id, config.csrf, config.csrf_token
    );

    let client = reqwest::Client::builder().build()?;
    let resp = client
        .post("https://api.live.bilibili.com/room/v1/Room/startLive")
        .body(body)
        .headers(header)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{:#?}", resp);
    Ok(())
}

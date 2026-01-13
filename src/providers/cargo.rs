use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CratesIoResponse {
    versions: Vec<CrateVersion>,
}

#[derive(Deserialize, Debug)]
struct CrateVersion {
    num: String,
    yanked: bool,
}

pub async fn fetch_versions(client: &Client, crate_name: &str) -> Result<Vec<String>> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let resp = client
        .get(&url)
        .header("User-Agent", "MCP-Agent/1.0")
        .send()
        .await?
        .json::<CratesIoResponse>()
        .await?;
    
    let versions = resp.versions
        .into_iter()
        .filter(|v| !v.yanked)
        .map(|v| v.num)
        .collect();
        
    Ok(versions)
}


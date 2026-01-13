use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct NpmResponse {
    #[serde(default)]
    versions: HashMap<String, serde_json::Value>,
}

pub async fn fetch_versions(client: &Client, package_name: &str) -> Result<Vec<String>> {
    let url = format!("https://registry.npmjs.org/{}", package_name);
    let resp = client
        .get(&url)
        .header("User-Agent", "MCP-Agent/1.0")
        .send()
        .await?;

    if resp.status().as_u16() == 404 {
         return Err(anyhow::anyhow!("NPM package not found: {}", package_name));
    }

    let body = resp.json::<NpmResponse>().await?;
    
    let mut versions: Vec<String> = body.versions
        .into_keys()
        .collect();
        
    // Simple sort
    versions.sort();
    versions.reverse();
    
    Ok(versions)
}

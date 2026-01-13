use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DockerHubTagsResponse {
    results: Vec<DockerTag>,
    #[allow(dead_code)]
    next: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DockerTag {
    name: String,
}

pub async fn fetch_versions(client: &Client, image_name: &str) -> Result<Vec<String>> {
    // Handle "ubuntu" -> "library/ubuntu"
    let full_image_name = if !image_name.contains('/') {
        format!("library/{}", image_name)
    } else {
        image_name.to_string()
    };

    let (namespace, repo) = match full_image_name.split_once('/') {
        Some((n, r)) => (n, r),
        None => return Err(anyhow::anyhow!("Invalid Docker image format")),
    };

    let url = format!("https://hub.docker.com/v2/repositories/{}/{}/tags?page_size=100", namespace, repo);
    
    let resp = client
        .get(&url)
        .header("User-Agent", "MCP-Agent/1.0")
        .send()
        .await?;
        
    if resp.status().as_u16() == 404 {
         return Err(anyhow::anyhow!("Docker image not found: {}", full_image_name));
    }

    let body = resp.json::<DockerHubTagsResponse>().await?;
    
    let versions = body.results
        .into_iter()
        .map(|t| t.name)
        .collect();
        
    Ok(versions)
}


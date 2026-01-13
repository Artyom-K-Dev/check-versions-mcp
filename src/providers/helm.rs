use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ArtifactHubResponse {
    available_versions: Vec<ArtifactHubVersion>,
}

#[derive(Deserialize, Debug)]
struct ArtifactHubVersion {
    version: String,
}

pub async fn fetch_versions(client: &Client, package_name: &str) -> Result<Vec<String>> {
    // Artifact Hub expects "repository/package" format for direct lookup
    let parts: Vec<&str> = package_name.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid Helm chart format. Expected 'repository/chart' (e.g. 'bitnami/postgresql')"));
    }
    
    let repo = parts[0];
    let name = parts[1];

    let url = format!("https://artifacthub.io/api/v1/packages/helm/{}/{}", repo, name);
    
    let resp = client
        .get(&url)
        .header("User-Agent", "MCP-Agent/1.0")
        .send()
        .await?;

    if resp.status().as_u16() == 404 {
         return Err(anyhow::anyhow!("Helm chart not found on Artifact Hub: {}", package_name));
    }

    let body = resp.json::<ArtifactHubResponse>().await?;
    
    let versions = body.available_versions
        .into_iter()
        .map(|v| v.version)
        .collect();
        
    Ok(versions)
}


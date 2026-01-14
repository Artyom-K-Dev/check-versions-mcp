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

    let text = resp.text().await?;
    parse_json(&text)
}

fn parse_json(json: &str) -> Result<Vec<String>> {
    let body: DockerHubTagsResponse = serde_json::from_str(json)?;
    let versions = body.results
        .into_iter()
        .map(|t| t.name)
        .collect();
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_docker_tags() {
        let json = r#"
        {
            "results": [
                { "name": "latest" },
                { "name": "1.0.0" },
                { "name": "alpine" }
            ]
        }
        "#;
        let versions = parse_json(json).unwrap();
        assert_eq!(versions.len(), 3);
        assert!(versions.contains(&"latest".to_string()));
    }
}


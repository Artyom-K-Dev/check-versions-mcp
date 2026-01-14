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

    let text = resp.text().await?;
    parse_json(&text)
}

fn parse_json(json: &str) -> Result<Vec<String>> {
    let body: NpmResponse = serde_json::from_str(json)?;
    
    let mut versions: Vec<String> = body.versions
        .into_keys()
        .collect();
        
    // Simple sort
    versions.sort();
    versions.reverse();
    
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_npm_versions() {
        let json = r#"
        {
            "versions": {
                "1.0.0": {},
                "1.0.1": {},
                "0.9.0": {}
            }
        }
        "#;
        let versions = parse_json(json).unwrap();
        assert_eq!(versions.len(), 3);
        assert!(versions.contains(&"1.0.0".to_string()));
        assert!(versions.contains(&"1.0.1".to_string()));
        assert!(versions.contains(&"0.9.0".to_string()));
    }
}

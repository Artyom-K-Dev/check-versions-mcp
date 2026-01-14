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
        .await?;

    let text = resp.text().await?;
    parse_json(&text)
}

fn parse_json(json: &str) -> Result<Vec<String>> {
    let resp: CratesIoResponse = serde_json::from_str(json)?;
    let versions = resp.versions
        .into_iter()
        .filter(|v| !v.yanked)
        .map(|v| v.num)
        .collect();
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_versions() {
        let json = r#"
        {
            "versions": [
                { "num": "1.0.0", "yanked": false },
                { "num": "1.0.1", "yanked": true },
                { "num": "0.9.0", "yanked": false }
            ]
        }
        "#;
        let versions = parse_json(json).unwrap();
        assert_eq!(versions.len(), 2);
        assert!(versions.contains(&"1.0.0".to_string()));
        assert!(!versions.contains(&"1.0.1".to_string()));
        assert!(versions.contains(&"0.9.0".to_string()));
    }
}


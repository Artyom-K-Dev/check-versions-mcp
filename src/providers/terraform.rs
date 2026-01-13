use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TerraformVersionsResponse {
    versions: Vec<TerraformVersion>,
}

#[derive(Deserialize, Debug)]
struct TerraformVersion {
    version: String,
}

#[derive(Deserialize, Debug)]
struct TerraformModuleVersionsResponse {
    modules: Vec<TerraformModuleVersionContainer>,
}

#[derive(Deserialize, Debug)]
struct TerraformModuleVersionContainer {
    versions: Vec<TerraformVersion>,
}

pub async fn fetch_versions(client: &Client, package_name: &str) -> Result<Vec<String>> {
    // Determine if it's a provider or module based on structure
    // Providers: namespace/name (e.g. hashicorp/aws) -> 2 parts
    // Modules: namespace/name/provider (e.g. terraform-aws-modules/vpc/aws) -> 3 parts
    
    let parts: Vec<&str> = package_name.split('/').collect();
    
    if parts.len() == 2 {
        fetch_provider_versions(client, parts[0], parts[1]).await
    } else if parts.len() == 3 {
        fetch_module_versions(client, parts[0], parts[1], parts[2]).await
    } else {
        Err(anyhow::anyhow!("Invalid Terraform package format. Expected 'namespace/name' for providers or 'namespace/name/provider' for modules."))
    }
}

async fn fetch_provider_versions(client: &Client, namespace: &str, name: &str) -> Result<Vec<String>> {
    let url = format!("https://registry.terraform.io/v1/providers/{}/{}/versions", namespace, name);
    
    let resp = client
        .get(&url)
        .header("User-Agent", "MCP-Agent/1.0")
        .send()
        .await?;

    if resp.status().as_u16() == 404 {
         return Err(anyhow::anyhow!("Terraform provider not found: {}/{}", namespace, name));
    }

    let body = resp.json::<TerraformVersionsResponse>().await?;
    
    let mut versions: Vec<String> = body.versions
        .into_iter()
        .map(|v| v.version)
        .collect();
    
    // Reverse to show latest first (usually API returns ascending)
    versions.reverse();
    
    Ok(versions)
}

async fn fetch_module_versions(client: &Client, namespace: &str, name: &str, provider: &str) -> Result<Vec<String>> {
    let url = format!("https://registry.terraform.io/v1/modules/{}/{}/{}/versions", namespace, name, provider);
    
    let resp = client
        .get(&url)
        .header("User-Agent", "MCP-Agent/1.0")
        .send()
        .await?;
        
    if resp.status().as_u16() == 404 {
         return Err(anyhow::anyhow!("Terraform module not found: {}/{}/{}", namespace, name, provider));
    }

    let body = resp.json::<TerraformModuleVersionsResponse>().await?;
    
    // Modules response structure is slightly different, usually contains a list of modules (though we requested one)
    // with a versions list in each.
    
    let mut all_versions = Vec::new();
    
    for module in body.modules {
        for v in module.versions {
            all_versions.push(v.version);
        }
    }
    
    all_versions.reverse();
    
    Ok(all_versions)
}


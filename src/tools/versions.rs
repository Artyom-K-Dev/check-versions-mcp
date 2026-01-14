use rmcp::{tool, tool_router, handler::server::tool::{ToolRouter, Parameters}, ErrorData, model::ErrorCode};
use schemars::JsonSchema;
use rmcp::serde::Deserialize;
use std::future::Future;
use crate::providers::{cargo, helm, docker, terraform, npm};

#[derive(Clone)]
pub struct VersionsTool {
    client: reqwest::Client,
    pub tool_router: ToolRouter<Self>,
}

#[derive(Deserialize, JsonSchema)]
struct VersionsArgs {
    package_name: String,
    package_manager: String,
    limit: Option<usize>,
}

#[tool_router]
impl VersionsTool {
    pub fn new() -> Self {
        let router = Self::tool_router();
        eprintln!("DEBUG: VersionsTool::new created router with {} tools", router.list_all().len());
        Self {
            client: reqwest::Client::new(),
            tool_router: router,
        }
    }

    #[tool(description = "Get versions for a package from various package managers (cargo, helm, docker, terraform, npm).")]
    pub async fn get_versions(&self, params: Parameters<VersionsArgs>) -> Result<String, ErrorData> {
        let args = params.0;
        let package_name = args.package_name;
        let package_manager = args.package_manager;
        let limit = args.limit.unwrap_or(25);

        let result = match package_manager.as_str() {
            "cargo" => cargo::fetch_versions(&self.client, &package_name).await,
            "helm" => helm::fetch_versions(&self.client, &package_name).await,
            "docker" => docker::fetch_versions(&self.client, &package_name).await,
            "terraform" => terraform::fetch_versions(&self.client, &package_name).await,
            "npm" => npm::fetch_versions(&self.client, &package_name).await,
            _ => return Err(ErrorData { 
                code: ErrorCode::INVALID_PARAMS, 
                message: format!("Unsupported package manager: {}. Valid values are: 'cargo', 'helm', 'docker', 'terraform', 'npm'.", package_manager).into(), 
                data: None 
            }),
        };

        match result {
            Ok(mut versions) => {
                versions.sort_by(|a, b| {
                    let pa = parse_version(a);
                    let pb = parse_version(b);
                    pb.cmp(&pa)
                });

                if versions.len() > limit {
                    versions.truncate(limit);
                }
                let text = serde_json::to_string(&versions).unwrap_or_default();
                Ok(text)
            },
            Err(e) => {
                Err(ErrorData { 
                    code: ErrorCode::INTERNAL_ERROR, 
                    message: e.to_string().into(), 
                    data: None 
                })
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum VersionComponent {
    Numeric(u64),
    String(String),
}

fn parse_version(v: &str) -> Vec<VersionComponent> {
    let v_clean = v.strip_prefix('v').unwrap_or(v);
    v_clean.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|part| {
            if let Ok(n) = part.parse::<u64>() {
                VersionComponent::Numeric(n)
            } else {
                VersionComponent::String(part.to_string())
            }
        })
        .collect()
}

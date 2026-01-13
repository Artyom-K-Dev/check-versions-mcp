use rmcp::{tool, tool_router, handler::server::tool::{ToolRouter, Parameters}, ErrorData, model::ErrorCode};
use schemars::JsonSchema;
use rmcp::serde::Deserialize;
use std::future::Future;
use crate::providers::{cargo, helm, docker, terraform};

#[derive(Clone)]
pub struct VersionsTool {
    client: reqwest::Client,
    pub tool_router: ToolRouter<Self>,
}

#[derive(Deserialize, JsonSchema)]
struct VersionsArgs {
    package_name: String,
    package_manager: String,
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

    #[tool(description = "Get versions for a package from various package managers (cargo, helm, docker, terraform).")]
    pub async fn get_versions(&self, params: Parameters<VersionsArgs>) -> Result<String, ErrorData> {
        let args = params.0;
        let package_name = args.package_name;
        let package_manager = args.package_manager;

        let result = match package_manager.as_str() {
            "cargo" => cargo::fetch_versions(&self.client, &package_name).await,
            "helm" => helm::fetch_versions(&self.client, &package_name).await,
            "docker" => docker::fetch_versions(&self.client, &package_name).await,
            "terraform" => terraform::fetch_versions(&self.client, &package_name).await,
            _ => return Err(ErrorData { 
                code: ErrorCode::INVALID_PARAMS, 
                message: format!("Unsupported package manager: {}. Valid values are: 'cargo', 'helm', 'docker', 'terraform'.", package_manager).into(), 
                data: None 
            }),
        };

        match result {
            Ok(versions) => {
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

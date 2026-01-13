use anyhow::Result;
use rmcp::{ServerHandler, model::ServerInfo, service::ServiceExt, transport};
use rmcp::model::{CallToolRequestParam, CallToolResult, ListToolsResult, PaginatedRequestParam, Implementation, ServerCapabilities, ToolsCapability};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::ErrorData;
use rmcp::handler::server::tool::ToolCallContext;

mod providers;
mod tools;

use crate::tools::versions::VersionsTool;

impl ServerHandler for VersionsTool {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "check-versions-mcp".into(),
                version: "0.1.0".into(),
            },
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    async fn call_tool(
        &self, 
        params: CallToolRequestParam, 
        req: RequestContext<RoleServer>
    ) -> Result<CallToolResult, ErrorData> {
        eprintln!("DEBUG: call_tool called");
        let context = ToolCallContext::new(self, params, req);
        self.tool_router.call(context).await
    }

    async fn list_tools(
        &self, 
        _params: Option<PaginatedRequestParam>, 
        _req: RequestContext<RoleServer>
    ) -> Result<ListToolsResult, ErrorData> {
        eprintln!("DEBUG: list_tools called (router)");
        let tools = self.tool_router.list_all();
        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("Panic: {:?}", info);
    }));
    env_logger::init();

    let tool = VersionsTool::new();
    let transport = transport::stdio();
    
    let service = tool.serve(transport).await?;
    service.waiting().await?;
    
    eprintln!("DEBUG: Server exited normally");
    Ok(())
}

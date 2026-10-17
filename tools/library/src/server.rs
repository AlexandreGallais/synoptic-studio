//! The MCP server of the library on `rmcp`, the official Rust SDK (`REF-RMCP`), over the `stdio`
//! transport (`REF-MCP-TRANSPORTS`): `tools/list` gives the three tools with the schemas they had,
//! `tools/call` answers each as the command line does. Declared in `.mcp.json`.

use std::future::{Future, ready};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
    ListToolsResult, PaginatedRequestParams, ServerCapabilities, ServerConfig, Tool,
};
use rmcp::service::{MaybeSendFuture, RequestContext};
use rmcp::{ErrorData, RoleServer, ServerHandler};
use rusqlite::Connection;
use serde_json::Value;

use crate::tools::{tool_list, tool_result};

/// The server: the open library, the project it runs for, and the folder whose checkout records
/// each call as a use of the tool (EN-102), `None` to record nothing.
pub struct Library {
    database: Mutex<Connection>,
    project: String,
    uses_folder: Option<PathBuf>,
}

impl Library {
    /// A server answering from an open library for a project, recording its calls in the
    /// checkout of a folder when one is given.
    #[must_use]
    pub fn new(database: Connection, project: String, uses_folder: Option<PathBuf>) -> Self {
        Self {
            database: Mutex::new(database),
            project,
            uses_folder,
        }
    }

    /// The answer of a tool call: its text, and whether it is an error; the call recorded as a
    /// use of `library-mcp`, the tool's name as its subcommand, exit code 1 for an error. Calls are
    /// answered one at a time, as the TypeScript server read one line after the other.
    #[must_use]
    pub fn answer(&self, name: &str, arguments: &Value) -> (String, bool) {
        let database = self
            .database
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut answer = (String::new(), false);

        tool_use::run_in(
            self.uses_folder.as_deref(),
            "library-mcp",
            name,
            false,
            || {
                answer = tool_result(&database, &self.project, name, arguments);
                u8::from(answer.1)
            },
        );

        answer
    }
}

/// The tools as `tools/list` gives them.
#[must_use]
pub fn tools() -> Vec<Tool> {
    tool_list()
        .into_iter()
        .map(|(name, description, schema)| {
            let Value::Object(schema) = schema else {
                unreachable!("each schema is an object")
            };

            Tool::new(name, description, Arc::new(schema))
        })
        .collect()
}

impl ServerHandler for Library {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("library", env!("CARGO_PKG_VERSION")))
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + MaybeSendFuture + '_ {
        ready(Ok(ListToolsResult::with_all_items(tools())))
    }

    // The answer is computed before the future is returned: a library call waits on SQLite or
    // `curl`, never on the runtime.
    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResponse, ErrorData>> + MaybeSendFuture + '_ {
        let arguments = request.arguments.map_or(Value::Null, Value::Object);
        let (text, is_error) = self.answer(&request.name, &arguments);
        let content = vec![ContentBlock::text(text)];

        ready(Ok(if is_error {
            CallToolResult::error(content)
        } else {
            CallToolResult::success(content)
        }
        .into()))
    }
}

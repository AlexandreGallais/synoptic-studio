//! MCP server of the local source library (`the records' conventions, area library`, ADR-0039), on `rmcp` over
//! `stdio`, declared in `.mcp.json`. Claude Code starts it in the project's folder;
//! `CLAUDE_PROJECT_DIR` names that folder when set, and each use records its project (EN-101).
//! The session is recorded as a use of `library-mcp serve`, and each call as a use of
//! `library-mcp <tool>` (EN-102).

use std::env;
use std::process::ExitCode;

use library::server::Library;
use library::store::{library_file_of_process, open_library};
use library::usage::project_of;
use rmcp::ServiceExt;
use rmcp::transport::stdio;

/// Serves the library until the client closes the input; the exit code.
async fn serve() -> u8 {
    let Some(file) = library_file_of_process() else {
        eprintln!("No library file: set AGENT_LIBRARY_FILE or HOME");
        return 1;
    };
    let database = match open_library(&file) {
        Ok(database) => database,
        Err(error) => {
            eprintln!(
                "The library cannot be opened at {}: {error}",
                file.display()
            );
            return 1;
        }
    };
    let directory = env::var("CLAUDE_PROJECT_DIR")
        .ok()
        .or_else(|| {
            env::current_dir()
                .ok()
                .map(|folder| folder.to_string_lossy().into_owned())
        })
        .unwrap_or_default();
    // Each call recorded as a use, in a release build only, as `tool_use::run` records.
    let uses_folder = if cfg!(debug_assertions) {
        None
    } else {
        env::current_dir().ok()
    };
    let server = Library::new(database, project_of(&directory).to_owned(), uses_folder);

    match server.serve(stdio()).await {
        Ok(running) => {
            // The client closing the input ends the server, whatever the reason given.
            let _ = running.waiting().await;
            0
        }
        Err(error) => {
            eprintln!("The library server did not start: {error}");
            1
        }
    }
}

/// One session of the server, on a runtime of one thread; the exit code.
fn session() -> u8 {
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime.block_on(serve()),
        Err(error) => {
            eprintln!("The library server has no runtime: {error}");
            1
        }
    }
}

fn main() -> ExitCode {
    tool_use::run("library-mcp", "serve", session)
}

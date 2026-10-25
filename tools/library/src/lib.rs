//! The local source library (`the records' conventions, area library`, ADR-0039): its SQLite store, the pages
//! fetched and cut into sections, the record of each use, the MCP server on `rmcp` and its command
//! line (EN-106). The behaviour is the one the TypeScript scripts had, so that the same store and
//! the same calls give the same answers.

pub mod chunks;
pub mod html;
pub mod pages;
pub mod server;
pub mod store;
pub mod text;
pub mod time;
pub mod tools;
pub mod usage;

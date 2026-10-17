//! A run's progress page (EN-087, research note 0024): one static HTML file written from the
//! backlog's feature tables, Git and `.git/metrics/`, that the main thread publishes as a claude.ai
//! Artifact at the run's start and after each merge, at the address kept in `.git/metrics/`.

pub mod address;
pub mod agents;
pub mod backlog;
pub mod commits;
pub mod page;
pub mod progress;

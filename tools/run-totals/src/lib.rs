//! The duration and tokens of a whole run (EN-092, research note 0024): the marker `/run` writes,
//! the transcripts' tokens read incrementally from it across compactions and new messages, and the
//! report at the run's end.

pub mod marker;
pub mod report;
pub mod tally;
pub mod totals;

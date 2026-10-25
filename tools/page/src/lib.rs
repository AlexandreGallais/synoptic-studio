//! One row or one section of a repository page (EN-105, research note 0025): the row of a table
//! found by the id of its first cell, the section found by its heading; an id alone is the
//! records', shown by `knowledge show` (`fallback`). A line scan of the standard library, so that
//! the agent reads the part it needs rather than the whole page.

pub mod fallback;
pub mod fence;
pub mod section;
pub mod table;

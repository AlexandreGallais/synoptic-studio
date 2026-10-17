//! The repository's one formatter (EN-073, F19.AC5, F19.AC12): oxfmt formats every file kind it
//! knows, then Markdown tables are printed without padding, so that a wider cell changes its own
//! row only (ADR-0068). oxfmt has no option for it: it pads every table.

pub mod ignore;
pub mod table;
pub mod targets;

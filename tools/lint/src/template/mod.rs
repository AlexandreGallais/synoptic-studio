//! The reader of Angular templates (F52, EN-163): a port of the lexer and the parser of
//! `@angular/compiler`'s `ml_parser` (REF-ANGULAR-ML-PARSER, MIT, under ADR-0076), so that the
//! template rules read a template as Angular's compiler reads it (REF-ANGULAR-TEMPLATES,
//! REF-ANGULAR-CONTROL-FLOW): `lexer` gives the tokens, `tree` the nodes, `bindings` what each
//! attribute, text and block binds, `expressions` each expression parsed by Oxc with its pipes.

pub mod bindings;
pub mod entities;
pub mod expressions;
pub mod lexer;
pub mod tags;
pub mod tree;

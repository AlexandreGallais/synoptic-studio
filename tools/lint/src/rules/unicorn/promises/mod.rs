//! Unicorn's rules on promises that oxlint lacks (EN-081), each one module with the cases of its
//! documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read from the tree alone, as
//! `ESLint` ran them here without type information. No scope turns one off: they run on every
//! TypeScript file.

pub mod calls;
pub mod no_async_iterator_callback;
pub mod no_async_promise_finally;
pub mod no_multiple_promise_resolver_calls;
pub mod prefer_await;
pub mod prefer_promise_static_methods;
pub mod prefer_promise_try;
pub mod prefer_then_catch;

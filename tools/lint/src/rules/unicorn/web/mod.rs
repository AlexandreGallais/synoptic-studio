//! Unicorn's rules on the web platform that oxlint lacks (EN-082): URLs, `Blob` and `File`, each
//! one module with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests,
//! read from the tree alone, as `ESLint` ran them here without type information. `NAMES` run on
//! every TypeScript file, `URL` and `Blob` being Node.js's too; `DOM_NAMES` only where a DOM
//! exists (`scopes::DOM_FILES`).

pub mod no_blob_to_file;
pub mod no_invalid_boolean_attribute_value;
pub mod no_invalid_url_protocol_comparison;
pub mod no_url_in_search_params;
pub mod prefer_location_assign;
pub mod prefer_url_can_parse;
pub mod prefer_url_href;
pub mod prefer_url_search_parameters;
pub mod require_css_escape;
pub mod urls;

use crate::rules::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &no_blob_to_file::NoBlobToFile,
    &no_invalid_boolean_attribute_value::NoInvalidBooleanAttributeValue,
    &no_invalid_url_protocol_comparison::NoInvalidUrlProtocolComparison,
    &no_url_in_search_params::NoUrlInSearchParams,
    &prefer_location_assign::PreferLocationAssign,
    &prefer_url_can_parse::PreferUrlCanParse,
    &prefer_url_href::PreferUrlHref,
    &prefer_url_search_parameters::PreferUrlSearchParameters,
    &require_css_escape::RequireCssEscape,
];

/// The names of those on for every TypeScript file.
pub const NAMES: &[&str] = &[
    "unicorn/no-blob-to-file",
    "unicorn/no-invalid-url-protocol-comparison",
    "unicorn/no-url-in-search-params",
    "unicorn/prefer-url-can-parse",
    "unicorn/prefer-url-href",
    "unicorn/prefer-url-search-parameters",
];

/// The names of those on for the files where a DOM exists.
pub const DOM_NAMES: &[&str] = &[
    "unicorn/no-invalid-boolean-attribute-value",
    "unicorn/prefer-location-assign",
    "unicorn/require-css-escape",
];

#[cfg(test)]
mod tests {
    use super::{DOM_NAMES, NAMES, RULES};

    #[test]
    fn names_each_rule_once_in_one_list() {
        let mut names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();
        let mut listed: Vec<&str> = NAMES.iter().chain(DOM_NAMES).copied().collect();
        names.sort_unstable();
        listed.sort_unstable();

        assert_eq!(listed, names);
    }
}

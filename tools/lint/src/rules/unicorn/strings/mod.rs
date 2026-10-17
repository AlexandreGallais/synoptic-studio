//! Unicorn's rules on strings, templates, escapes and regular expressions that oxlint lacks
//! (EN-080; with `no-invalid-character-comparison`, EN-095), each one module with the cases of
//! its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read from the syntax alone. No
//! scope turns one off: they run on every TypeScript file. `prefer-escaped-irregular-whitespace`,
//! `prefer-string-match-all` and `string-content` are not carried: what they check is refused
//! elsewhere or not configured.

pub mod no_incorrect_template_string_interpolation;
pub mod no_invalid_character_comparison;
pub mod no_unnecessary_string_trim;
pub mod no_unsafe_string_replacement;
pub mod prefer_literal_ascii;
pub mod prefer_regexp_escape;
pub mod prefer_short_escape_sequences;
pub mod prefer_single_replace;
pub mod prefer_split_limit;
pub mod prefer_string_pad_start_end;
pub mod prefer_string_repeat;
pub mod prefer_unicode_code_point_escapes;

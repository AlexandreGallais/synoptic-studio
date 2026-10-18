//! Tests of the letter cases (EN-108): `es-toolkit` 1.52.0's words and conversions, the examples
//! of its documentation first, each other value worked out by its pattern's alternatives.

use git_hooks::case::{
    Case, camel_case, is_in_case, start_case, starts_with_cased_letter, to_case, upper_first, words,
};

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|&item| item.to_owned()).collect()
}

#[test]
fn splits_words_as_the_documentation_shows() {
    // `words('fred, barney, & pebbles')`, from `words.ts`.
    assert_eq!(
        words("fred, barney, & pebbles"),
        strings(&["fred", "barney", "pebbles"])
    );
    // An upper case run ends before the capital of a lower case word (`upper_word`'s look-ahead).
    assert_eq!(
        words("XMLHttpRequest"),
        strings(&["XML", "Http", "Request"])
    );
    assert_eq!(words("SubJect"), strings(&["Sub", "Ject"]));
    assert_eq!(words("sub_ject"), strings(&["sub", "ject"]));
    // `1ST` is an ordinal; `11TH` is not (`(?![123])\dTH` refuses the 1 before TH), so 11 and TH.
    assert_eq!(words("1ST place"), strings(&["1ST", "place"]));
    assert_eq!(
        words("the 2nd and 11TH"),
        strings(&["the", "2nd", "and", "11", "TH"])
    );
    // A hyphenated id: letters, then digits, then the next word.
    assert_eq!(words("SP-023 adds"), strings(&["SP", "023", "adds"]));
    // An emoji is a word of its own.
    assert_eq!(words("a😀b"), strings(&["a", "😀", "b"]));
    // A titlecase letter belongs to no alternative: skipped.
    assert_eq!(words("ǅx"), strings(&["x"]));
    assert_eq!(words(""), Vec::<String>::new());
}

#[test]
fn converts_to_start_and_camel_case_as_the_documentation_shows() {
    // From `startCase.ts`.
    assert_eq!(start_case("hello world"), "Hello World");
    assert_eq!(start_case("HELLO WORLD"), "HELLO WORLD");
    assert_eq!(start_case("hello-world"), "Hello World");
    assert_eq!(start_case("hello_world"), "Hello World");
    // From `camelCase.ts`.
    assert_eq!(camel_case("camelCase"), "camelCase");
    assert_eq!(camel_case("some whitespace"), "someWhitespace");
    assert_eq!(camel_case("hyphen-text"), "hyphenText");
    assert_eq!(camel_case("HTTPRequest"), "httpRequest");
    // Apostrophes go before the words are split (`normalizeForCase`).
    assert_eq!(start_case("don't stop"), "Dont Stop");
    // `deburr`'s table: ß becomes ss.
    assert_eq!(start_case("straße"), "Strasse");
    // Pascal case: camel case, its first letter in upper case.
    assert_eq!(to_case("sub ject", Case::Pascal), "SubJect");
    assert_eq!(to_case("", Case::Pascal), "");
}

#[test]
fn upper_cases_the_first_character_only() {
    assert_eq!(upper_first("fred"), "Fred");
    assert_eq!(upper_first("FRED"), "FRED");
    assert_eq!(upper_first(""), "");
    // A character of two UTF-16 units is left as it is, as JavaScript's `substring(0, 1)` cuts it.
    assert_eq!(upper_first("𐐨x"), "𐐨x");
}

#[test]
fn tests_a_case_its_quoted_parts_removed() {
    // A sentence-case subject: upper case of its first letter changes nothing.
    assert!(is_in_case("Add a rule", Case::Sentence));
    assert!(!is_in_case("add a rule", Case::Sentence));
    // The quoted proper name is removed first, then the rest is trimmed: "add the" is not in
    // sentence case.
    assert!(!is_in_case(
        "add the `Eslint` configuration",
        Case::Sentence
    ));
    // Nothing left, or a digit first: in every case.
    assert!(is_in_case("`Eslint`", Case::Sentence));
    assert!(is_in_case("1.0.0", Case::Upper));
    assert!(is_in_case("SUBJECT", Case::Upper));
    assert!(is_in_case("Sub Ject", Case::Start));
    assert!(!is_in_case("Sub ject", Case::Start));
    assert!(is_in_case("SubJect", Case::Pascal));
    assert!(is_in_case("type", Case::Lower));
    assert!(!is_in_case("tYpE", Case::Lower));
}

#[test]
fn finds_a_cased_letter_first() {
    assert!(starts_with_cased_letter("add"));
    assert!(starts_with_cased_letter("Тема"));
    assert!(starts_with_cased_letter("ǅx"));
    assert!(!starts_with_cased_letter("这是一次提交"));
    assert!(!starts_with_cased_letter("`code`"));
    assert!(!starts_with_cased_letter("1.0.0"));
    assert!(!starts_with_cased_letter(""));
}

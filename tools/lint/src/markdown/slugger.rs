//! The anchors GitHub gives headings, as `github-slugger` 2.0 writes them for
//! `no-missing-link-fragments` (`REF-GITHUB-SLUGGER`): the text lower-cased, every character
//! but letters, marks, digits, connectors, hyphens and spaces removed, each space turned to a
//! hyphen; a slug met again gets `-1`, `-2`… after it.

use std::collections::HashMap;

/// The slugs given so far on a page, and how many times each was asked again.
#[derive(Default)]
pub struct Slugger {
    occurrences: HashMap<String, usize>,
}

/// A text's slug, without the count of the slugs given before (`slug`). The characters kept are
/// those of a JavaScript identifier's continuation (`ID_Continue`), the set `github-slugger`'s
/// generated pattern keeps, with the hyphen and the space.
#[must_use]
pub fn slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|character| {
            matches!(character, ' ' | '-')
                || (markdown::id_cont(*character, false)
                    && !matches!(character, '\u{200c}' | '\u{200d}'))
        })
        .map(|character| if character == ' ' { '-' } else { character })
        .collect()
}

impl Slugger {
    /// A text's slug, made unique on the page (`BananaSlug.slug`).
    pub fn slug(&mut self, text: &str) -> String {
        let original = slug(text);
        let mut result = original.clone();

        while self.occurrences.contains_key(&result) {
            let count = self.occurrences.entry(original.clone()).or_insert(0);
            *count += 1;
            result = format!("{original}-{count}");
        }
        self.occurrences.insert(result.clone(), 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Slugger, slug};

    #[test]
    fn writes_github_s_anchors() {
        // The cases of github-slugger's README and tests.
        assert_eq!(slug("foo bar baz"), "foo-bar-baz");
        assert_eq!(slug("Hello, World!"), "hello-world");
        assert_eq!(slug("`local/barrel-exports`"), "localbarrel-exports");
        assert_eq!(slug("Ça va — très bien"), "ça-va--très-bien");
        assert_eq!(slug("snake_case 2² €"), "snake_case-2-");
        assert_eq!(slug("I ♥ 🦄"), "i--");
    }

    #[test]
    fn numbers_a_slug_met_again() {
        let mut slugger = Slugger::default();

        assert_eq!(
            ["foo", "foo", "foo-1", "foo"].map(|text| slugger.slug(text)),
            ["foo", "foo-1", "foo-1-1", "foo-2"]
        );
    }
}

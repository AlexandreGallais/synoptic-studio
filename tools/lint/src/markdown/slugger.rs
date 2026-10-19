//! The anchors GitHub gives headings, as `github-slugger` 2.0 writes them for
//! `no-missing-link-fragments` (`REF-GITHUB-SLUGGER`): the text lower-cased, every character
//! but letters, marks, digits, connectors, hyphens and spaces removed, each space turned to a
//! hyphen; a slug met again gets `-1`, `-2`… after it. And the anchors the site gives them,
//! as `VitePress` 2.0.0-alpha.20 writes them (`REF-VITEPRESS-SLUGIFY`, EN-140): `@mdit-vue/shared`'s
//! `slugify`, made unique by `markdown-it-anchor`.

use std::collections::{HashMap, HashSet};

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

/// The characters outside ASCII that `normalize("NFKD")` leaves as they are, among those the
/// repository's headings hold, each checked with Node.js 24's `String.prototype.normalize` on
/// 2026-10-19.
const NFKD_UNCHANGED: [char; 5] = ['—', '“', '”', '‘', '’'];

/// The characters outside ASCII that `normalize("NFKD")` decomposes, among those the repository's
/// headings hold, written as the site's slug keeps them, their combining marks removed: `é` is
/// `e` and U+0301, `ⁱ` is `i` (Node.js 24, 2026-10-19).
const NFKD_DECOMPOSED: [(char, char); 2] = [('é', 'e'), ('ⁱ', 'i')];

/// The characters the site's slug turns into a hyphen, a run of them into one: JavaScript's `\s`
/// and ``~`!@#$%^&*()-_+=[]{}|\;:"'“”‘’<>,.?/`` (`rSpecial`).
fn is_site_special(character: char) -> bool {
    character.is_whitespace() || "~`!@#$%^&*()-_+=[]{}|\\;:\"'“”‘’<>,.?/".contains(character)
}

/// A text's anchor on the site (`slugify`): decomposed (`NFKD`), its combining marks
/// (U+0300–U+036F) and control characters removed, each run of special characters turned into a
/// hyphen, the hyphens at both ends removed, an underscore before a leading digit, lower-cased.
/// `None` when the text holds a character outside ASCII whose decomposition this table does not
/// know.
#[must_use]
pub fn site_slug(text: &str) -> Option<String> {
    let mut decomposed = String::new();

    for character in text.chars() {
        if character.is_ascii() || NFKD_UNCHANGED.contains(&character) {
            decomposed.push(character);
        } else if let Some((_, base)) = NFKD_DECOMPOSED.iter().find(|(from, _)| *from == character)
        {
            decomposed.push(*base);
        } else if !('\u{300}'..='\u{36f}').contains(&character) {
            return None;
        }
    }

    let mut hyphenated = String::new();

    for character in decomposed.chars().filter(|character| *character >= ' ') {
        if !is_site_special(character) {
            hyphenated.push(character);
        } else if !hyphenated.ends_with('-') {
            hyphenated.push('-');
        }
    }

    let trimmed = hyphenated.trim_matches('-');
    let prefix = if trimmed.starts_with(|first: char| first.is_ascii_digit()) {
        "_"
    } else {
        ""
    };

    Some(format!("{prefix}{trimmed}").to_lowercase())
}

/// The anchors given so far on a page of the site (`markdown-it-anchor`'s `uniqueSlug`).
#[derive(Default)]
pub struct SiteSlugger {
    taken: HashSet<String>,
}

impl SiteSlugger {
    /// A text's anchor on the site, made unique on the page: `-1`, `-2`… after it until one is
    /// free; `None` as [`site_slug`] gives it.
    pub fn slug(&mut self, text: &str) -> Option<String> {
        let original = site_slug(text)?;
        let mut result = original.clone();
        let mut count = 1;

        while self.taken.contains(&result) {
            result = format!("{original}-{count}");
            count += 1;
        }
        self.taken.insert(result.clone());
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::{SiteSlugger, Slugger, site_slug, slug};

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

    #[test]
    fn f33_ac2_writes_the_site_s_anchors() {
        // ", " and "!" are special: "Hello-World-", the end hyphen trimmed, lower-cased.
        assert_eq!(site_slug("Hello, World!").as_deref(), Some("hello-world"));
        // The slash is special where GitHub drops it: "local-barrel-exports", not
        // "localbarrel-exports".
        assert_eq!(
            site_slug("local/barrel-exports").as_deref(),
            Some("local-barrel-exports")
        );
        // The em dash is kept between the hyphens of its spaces: "step-2-—-dividing-by-2i", ⁱ
        // decomposed to i, where GitHub writes "step-2--dividing-by-2ⁱ".
        assert_eq!(
            site_slug("Step 2 — Dividing by 2ⁱ").as_deref(),
            Some("step-2-—-dividing-by-2i")
        );
        // A leading digit takes an underscore: "2. Logged" is "2-Logged", then "_2-logged".
        assert_eq!(site_slug("2. Logged").as_deref(), Some("_2-logged"));
        // The underscore is special, the accent removed, a control character dropped.
        assert_eq!(site_slug("snake_case").as_deref(), Some("snake-case"));
        assert_eq!(site_slug("Café\u{1}").as_deref(), Some("cafe"));
        assert_eq!(site_slug("e\u{301}t\u{e9}").as_deref(), Some("ete"));
        // Only specials: nothing left once the hyphens are trimmed.
        assert_eq!(site_slug(" -- ").as_deref(), Some(""));
        // A character whose decomposition the table does not know: not computed.
        assert_eq!(site_slug("I ♥ it"), None);
    }

    #[test]
    fn f33_ac2_numbers_a_site_anchor_met_again() {
        let mut slugger = SiteSlugger::default();

        // "foo-1" is taken when the second "foo" asks again: "foo-1-1", then "foo-2".
        assert_eq!(
            ["foo", "foo", "foo-1", "foo"].map(|text| slugger.slug(text)),
            ["foo", "foo-1", "foo-1-1", "foo-2"].map(|slug| Some(slug.to_owned()))
        );
        assert_eq!(slugger.slug("♥"), None);
    }
}

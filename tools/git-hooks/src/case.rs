//! The letter cases commitlint's `subject-case` and `type-case` test (EN-108): a text is in a case
//! when converting it to that case leaves it unchanged (`@commitlint/ensure` 21.2.3, `case.js` and
//! `to-case.js`).
//!
//! The conversions are ported from `es-toolkit` 1.52.0 (MIT, Copyright (c) 2024 Viva Republica,
//! Inc.), as `@commitlint/ensure` calls them from `es-toolkit/compat`: `upperFirst`, `startCase`,
//! `camelCase`, `words`, `deburr`, `normalizeForCase`. Two differences, each unable to change the
//! verdict of the project's `subject-case` (below): `deburr` drops the combining marks and maps
//! its letters without first decomposing the text to NFD, which no crate of the workspace does;
//! and JavaScript's lone UTF-16 halves are the whole character here, kept as it is where
//! JavaScript changes the first half only.
//!
//! Why the verdict holds: `subject-case` never lets a subject start with a cased letter that
//! upper case leaves unchanged (sentence case), and a start, Pascal or upper case text starts with
//! such a letter or is empty, whatever `deburr` did to its accents.

use std::sync::LazyLock;

use regex::Regex;

/// A regular expression testing one character.
fn class(pattern: &str) -> Regex {
    Regex::new(&format!("^{pattern}$")).unwrap_or_else(|_| unreachable!("a valid class"))
}

/// `\p{Lu}`.
static UPPER: LazyLock<Regex> = LazyLock::new(|| class(r"\p{Lu}"));
/// `\p{Ll}`.
static LOWER: LazyLock<Regex> = LazyLock::new(|| class(r"\p{Ll}"));
/// `[\p{Lm}\p{Lo}]`, the start of `rMisc`.
static MISC: LazyLock<Regex> = LazyLock::new(|| class(r"[\p{Lm}\p{Lo}]"));
/// `\p{M}`.
static MARK: LazyLock<Regex> = LazyLock::new(|| class(r"\p{M}"));
/// `rUnicodeBreak`: separators, punctuation and the Latin-1 characters that are not letters.
static BREAK: LazyLock<Regex> =
    LazyLock::new(|| class(r"[\p{Z}\p{P}\x00-\x2f\x3a-\x40\x5b-\x60\x7b-\xbf\xd7\xf7]"));
/// `\p{Emoji_Presentation}`.
static EMOJI: LazyLock<Regex> = LazyLock::new(|| class(r"\p{Emoji_Presentation}"));
/// `\p{Extended_Pictographic}`.
static PICTOGRAPHIC: LazyLock<Regex> = LazyLock::new(|| class(r"\p{Extended_Pictographic}"));
/// The cased letters `subject-case` requires first: `[\p{Ll}\p{Lu}\p{Lt}]`.
static CASED: LazyLock<Regex> = LazyLock::new(|| class(r"[\p{Ll}\p{Lu}\p{Lt}]"));
/// A quoted part, removed before a case is tested: it may hold a proper name.
static QUOTED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"`[^\n\r\x{2028}\x{2029}]*?`|"[^\n\r\x{2028}\x{2029}]*?"|'[^\n\r\x{2028}\x{2029}]*?'"#,
    )
    .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// Whether a character belongs to a class.
fn is(class: &Regex, character: char) -> bool {
    let mut buffer = [0; 4];

    class.is_match(character.encode_utf8(&mut buffer))
}

/// A case commitlint can test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Case {
    /// `lower-case`.
    Lower,
    /// `upper-case`.
    Upper,
    /// `sentence-case`: the first character in upper case.
    Sentence,
    /// `start-case`: each word capitalised.
    Start,
    /// `pascal-case`.
    Pascal,
}

impl Case {
    /// Its name in commitlint's configuration.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Lower => "lower-case",
            Self::Upper => "upper-case",
            Self::Sentence => "sentence-case",
            Self::Start => "start-case",
            Self::Pascal => "pascal-case",
        }
    }
}

/// Whether a text starts with a cased letter, the condition of `subject-case`.
#[must_use]
pub fn starts_with_cased_letter(text: &str) -> bool {
    text.chars().next().is_some_and(|first| is(&CASED, first))
}

/// Whether a character takes two UTF-16 units, which JavaScript's `[0]` and `substring(0, 1)` cut.
fn is_astral(character: char) -> bool {
    character.len_utf16() == 2
}

/// `upperFirst`: the first character in upper case, the rest as it is.
#[must_use]
pub fn upper_first(text: &str) -> String {
    let mut characters = text.chars();

    match characters.next() {
        Some(first) if !is_astral(first) => first.to_uppercase().chain(characters).collect(),
        _ => text.to_owned(),
    }
}

/// `capitalize`: the first character in upper case, the rest in lower case.
fn capitalize(word: &str) -> String {
    let mut characters = word.chars();

    match characters.next() {
        Some(first) => {
            let head: String = if is_astral(first) {
                first.to_string()
            } else {
                first.to_uppercase().collect()
            };

            head + &characters.as_str().to_lowercase()
        }
        None => String::new(),
    }
}

/// The letters `deburr` maps to Latin letters.
const DEBURR: [(char, &str); 29] = [
    ('Æ', "Ae"),
    ('Ð', "D"),
    ('Ø', "O"),
    ('Þ', "Th"),
    ('ß', "ss"),
    ('æ', "ae"),
    ('ð', "d"),
    ('ø', "o"),
    ('þ', "th"),
    ('Đ', "D"),
    ('đ', "d"),
    ('Ħ', "H"),
    ('ħ', "h"),
    ('ı', "i"),
    ('Ĳ', "IJ"),
    ('ĳ', "ij"),
    ('ĸ', "k"),
    ('Ŀ', "L"),
    ('ŀ', "l"),
    ('Ł', "L"),
    ('ł', "l"),
    ('ŉ', "'n"),
    ('Ŋ', "N"),
    ('ŋ', "n"),
    ('Œ', "Oe"),
    ('œ', "oe"),
    ('Ŧ', "T"),
    ('ŧ', "t"),
    ('ſ', "s"),
];

/// `deburr` without its NFD step: the combining marks of three blocks dropped, the letters of
/// the table mapped.
fn deburr(text: &str) -> String {
    text.chars()
        .filter(|character| {
            !matches!(character, '\u{300}'..='\u{36f}' | '\u{20d0}'..='\u{20ff}' | '\u{fe20}'..='\u{fe2f}')
        })
        .map(|character| {
            DEBURR
                .iter()
                .find(|(letter, _)| *letter == character)
                .map_or_else(|| character.to_string(), |(_, latin)| (*latin).to_owned())
        })
        .collect()
}

/// `normalizeForCase`: apostrophes removed.
fn normalize_for_case(text: &str) -> String {
    text.chars()
        .filter(|&character| character != '\'' && character != '\u{2019}')
        .collect()
}

/// The end of the `(?:[\p{Lm}\p{Lo}]\p{M}*)` starting at `at`, if one does.
fn misc_end(characters: &[char], at: usize) -> Option<usize> {
    let first = *characters.get(at)?;

    is(&MISC, first).then(|| {
        at + 1
            + characters[at + 1..]
                .iter()
                .take_while(|&&character| is(&MARK, character))
                .count()
    })
}

/// The end of one `rUnicodeMiscUpper` (`\p{Lu}` or a misc letter) at `at`.
fn misc_upper_end(characters: &[char], at: usize) -> Option<usize> {
    match characters.get(at) {
        Some(&character) if is(&UPPER, character) => Some(at + 1),
        _ => misc_end(characters, at),
    }
}

/// The end of one `rUnicodeMiscLower` (`\p{Ll}` or a misc letter) at `at`.
fn misc_lower_end(characters: &[char], at: usize) -> Option<usize> {
    match characters.get(at) {
        Some(&character) if is(&LOWER, character) => Some(at + 1),
        _ => misc_end(characters, at),
    }
}

/// The ends after each of the tokens repeated from `at`, in order.
fn token_ends(
    characters: &[char],
    at: usize,
    token: fn(&[char], usize) -> Option<usize>,
) -> Vec<usize> {
    let mut ends = Vec::new();
    let mut position = at;

    while let Some(end) = token(characters, position) {
        ends.push(end);
        position = end;
    }

    ends
}

/// Whether a character at a position is a break, an upper case letter or the end: the look-ahead
/// of the first alternative.
fn break_upper_or_end(characters: &[char], at: usize) -> bool {
    characters
        .get(at)
        .is_none_or(|&character| is(&BREAK, character) || is(&UPPER, character))
}

/// `\p{Lu}?\p{Ll}+(?=break|\p{Lu}|$)`.
fn lower_word(characters: &[char], at: usize) -> Option<usize> {
    let starts: &[usize] = if characters.get(at).is_some_and(|&first| is(&UPPER, first)) {
        &[1, 0]
    } else {
        &[0]
    };

    starts.iter().find_map(|&skip| {
        let from = at + skip;
        let run = characters[from.min(characters.len())..]
            .iter()
            .take_while(|&&character| is(&LOWER, character))
            .count();

        (1..=run)
            .rev()
            .map(|length| from + length)
            .find(|&end| break_upper_or_end(characters, end))
    })
}

/// `rUnicodeMiscUpper+(?=break|\p{Lu}rUnicodeMiscLower|$)`.
fn upper_word(characters: &[char], at: usize) -> Option<usize> {
    token_ends(characters, at, misc_upper_end)
        .into_iter()
        .rev()
        .find(|&end| match characters.get(end) {
            None => true,
            Some(&next) => {
                is(&BREAK, next)
                    || (is(&UPPER, next) && misc_lower_end(characters, end + 1).is_some())
            }
        })
}

/// `\p{Lu}?rUnicodeMiscLower+`.
fn mixed_word(characters: &[char], at: usize) -> Option<usize> {
    let after_upper = characters
        .get(at)
        .filter(|&&first| is(&UPPER, first))
        .and_then(|_| {
            token_ends(characters, at + 1, misc_lower_end)
                .last()
                .copied()
        });

    after_upper.or_else(|| token_ends(characters, at, misc_lower_end).last().copied())
}

/// `\p{Lu}+`.
fn capitals(characters: &[char], at: usize) -> Option<usize> {
    let run = characters[at..]
        .iter()
        .take_while(|&&character| is(&UPPER, character))
        .count();

    (run > 0).then_some(at + run)
}

/// Whether a character is a word character of JavaScript's `\b`, ASCII only.
fn is_ascii_word(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

/// `\d*(?:1ST|2ND|3RD|(?![123])\dTH)(?=\b|[a-z_])`, or its lower-case twin with `[A-Z_]`.
fn ordinal(characters: &[char], at: usize, upper: bool) -> Option<usize> {
    let suffixes: [&str; 3] = if upper {
        ["1ST", "2ND", "3RD"]
    } else {
        ["1st", "2nd", "3rd"]
    };
    let th: [char; 2] = if upper { ['T', 'H'] } else { ['t', 'h'] };
    let digits = characters[at..]
        .iter()
        .take_while(|character| character.is_ascii_digit())
        .count();
    let follows = |end: usize| match characters.get(end) {
        None => true,
        Some(&next) => {
            !is_ascii_word(next)
                || next == '_'
                || if upper {
                    next.is_ascii_lowercase()
                } else {
                    next.is_ascii_uppercase()
                }
        }
    };
    let starts_with = |position: usize, text: &str| {
        text.chars()
            .enumerate()
            .all(|(offset, letter)| characters.get(position + offset) == Some(&letter))
    };

    (0..=digits).rev().find_map(|count| {
        let position = at + count;
        let lettered = suffixes
            .iter()
            .find(|suffix| starts_with(position, suffix))
            .map(|_| position + 3);
        let numbered = characters
            .get(position)
            .filter(|digit| digit.is_ascii_digit() && !matches!(digit, '1' | '2' | '3'))
            .filter(|_| characters.get(position + 1..position + 3) == Some(&th[..]))
            .map(|_| position + 3);

        [lettered, numbered]
            .into_iter()
            .flatten()
            .find(|&end| follows(end))
    })
}

/// `\d+`, ASCII digits as JavaScript's `\d`.
fn number(characters: &[char], at: usize) -> Option<usize> {
    let run = characters[at..]
        .iter()
        .take_while(|character| character.is_ascii_digit())
        .count();

    (run > 0).then_some(at + run)
}

/// One emoji character.
fn emoji(characters: &[char], at: usize) -> Option<usize> {
    let character = *characters.get(at)?;

    (is(&EMOJI, character) || is(&PICTOGRAPHIC, character)).then_some(at + 1)
}

/// `words`: the words of a text, found by `rUnicodeWord`'s alternatives in their order at each
/// position, a position matching none skipped.
#[must_use]
pub fn words(text: &str) -> Vec<String> {
    let characters: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    let mut at = 0;

    while at < characters.len() {
        let end = lower_word(&characters, at)
            .or_else(|| upper_word(&characters, at))
            .or_else(|| mixed_word(&characters, at))
            .or_else(|| capitals(&characters, at))
            .or_else(|| ordinal(&characters, at, true))
            .or_else(|| ordinal(&characters, at, false))
            .or_else(|| number(&characters, at))
            .or_else(|| emoji(&characters, at));

        match end {
            Some(end) => {
                found.push(characters[at..end].iter().collect());
                at = end;
            }
            None => at += 1,
        }
    }

    found
}

/// `startCase`: each word capitalised, a word in capitals kept, joined by spaces.
#[must_use]
pub fn start_case(text: &str) -> String {
    words(normalize_for_case(&deburr(text)).trim())
        .iter()
        .map(|word| {
            if *word == word.to_uppercase() {
                word.clone()
            } else {
                capitalize(word)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// `camelCase`: the first word in lower case, the others capitalised, joined.
#[must_use]
pub fn camel_case(text: &str) -> String {
    let found = words(&normalize_for_case(&deburr(text)));

    match found.split_first() {
        Some((first, rest)) => {
            first.to_lowercase() + &rest.iter().map(|word| capitalize(word)).collect::<String>()
        }
        None => String::new(),
    }
}

/// A text converted to a case (`toCase`).
#[must_use]
pub fn to_case(text: &str, case: Case) -> String {
    match case {
        Case::Lower => text.to_lowercase(),
        Case::Upper => text.to_uppercase(),
        Case::Sentence => upper_first(text),
        Case::Start => start_case(text),
        Case::Pascal => upper_first(&camel_case(text)),
    }
}

/// Whether a text is in a case (`ensureCase`): its quoted parts removed and the rest trimmed, the
/// conversion leaves it unchanged, or gives nothing or a text starting with a digit.
#[must_use]
pub fn is_in_case(text: &str, case: Case) -> bool {
    let input = QUOTED.replace_all(text, "");
    let input = input.trim();
    let converted = to_case(input, case);

    converted.is_empty()
        || converted.starts_with(|first: char| first.is_ascii_digit())
        || converted == input
}

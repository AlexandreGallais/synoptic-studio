//! The ids a text cites (research note 0035, Its tables), which `check-ids` checks against the
//! records.

/// Whether a word is an id the project cites: `REF-` and upper-case parts (`REF-GIT-BRANCH`),
/// `DERIV-` and lower-case ones (`DERIV-bit`), an epic or a feature (`E49`, `F48`), or two
/// capitals or more, a dash and three digits or more, then dashed numbers (`EN-152`, `ADR-0081`,
/// `JOURNAL-2026-10-25-3`).
#[must_use]
pub fn is_citation(word: &str) -> bool {
    let all = |part: &str, accepted: fn(&u8) -> bool| {
        !part.is_empty() && part.bytes().all(|byte| accepted(&byte))
    };
    let upper = |part: &&str| {
        all(part, |byte| {
            byte.is_ascii_uppercase() || byte.is_ascii_digit()
        })
    };
    let lower = |part: &&str| {
        all(part, |byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit()
        })
    };
    let digits = |part: &str| all(part, u8::is_ascii_digit);
    let parts: Vec<&str> = word.split('-').collect();

    match parts.as_slice() {
        ["REF", rest @ ..] => !rest.is_empty() && rest.iter().all(upper),
        ["DERIV" | "RULE", rest @ ..] => !rest.is_empty() && rest.iter().all(lower),
        [single] => single
            .split_at_checked(1)
            .is_some_and(|(letter, number)| matches!(letter, "E" | "F") && digits(number)),
        [kind, number, rest @ ..] => {
            kind.len() >= 2
                && all(kind, u8::is_ascii_uppercase)
                && number.len() >= 3
                && digits(number)
                && rest.iter().all(|part| digits(part))
        }
        [] => false,
    }
}

/// The ids a text cites, each once, in the order first met: its words, cut at every character but
/// a letter, a digit or a dash, their outer dashes left out, that [`is_citation`] accepts.
#[must_use]
pub fn citations(text: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();

    for word in
        text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
    {
        let word = word.trim_matches('-');

        if is_citation(word) && !found.iter().any(|known| known == word) {
            found.push(word.to_owned());
        }
    }
    found
}

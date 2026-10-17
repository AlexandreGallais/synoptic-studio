//! The order of `TypeDoc`'s `alphabetical-ignoring-documents` sort (`REF-TYPEDOC-OPTIONS`),
//! which compares names with `localeCompare`: letters without regard to case, `_` before
//! digits and letters, then lower case before upper case.

use std::cmp::Ordering;

/// The rank of a character at the first level of comparison: punctuation, digits, letters
/// without case.
fn primary(character: char) -> (u8, char) {
    match character {
        '0'..='9' => (1, character),
        _ if character.is_alphabetic() => (2, character.to_ascii_lowercase()),
        _ => (0, character),
    }
}

/// Two names compared as `localeCompare` does for the identifiers of the library.
#[must_use]
pub fn compare(left: &str, right: &str) -> Ordering {
    let first = left.chars().map(primary).cmp(right.chars().map(primary));
    first.then_with(|| {
        let case = |text: &str| text.chars().map(char::is_uppercase).collect::<Vec<_>>();
        case(left).cmp(&case(right))
    })
}

/// Items sorted by their name.
#[must_use]
pub fn sorted_by_name<T>(items: &[T], name: impl Fn(&T) -> &str) -> Vec<&T> {
    let mut sorted: Vec<&T> = items.iter().collect();
    sorted.sort_by(|left, right| compare(name(left), name(right)));
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_case_first() {
        assert_eq!(
            compare("angledHandle", "angleFromDirection"),
            Ordering::Less
        );
        assert_eq!(compare("Alignment", "add"), Ordering::Greater);
        assert_eq!(compare("ByteRgb", "bytesToHex"), Ordering::Less);
    }

    #[test]
    fn puts_the_underscore_before_letters_and_digits() {
        assert_eq!(
            compare("DEGREES_PER_TURN", "DEGREES_PERCENT"),
            Ordering::Less
        );
        assert_eq!(compare("A_B", "A1"), Ordering::Less);
        assert_eq!(compare("A1", "AB"), Ordering::Less);
    }

    #[test]
    fn puts_a_prefix_first_then_lower_case() {
        assert_eq!(compare("hue", "hueSlider"), Ordering::Less);
        assert_eq!(compare("text", "Text"), Ordering::Less);
        assert_eq!(compare("Text", "Text"), Ordering::Equal);
    }

    #[test]
    fn sorts_items_by_name() {
        let items = ["b", "C", "a"];
        assert_eq!(sorted_by_name(&items, |item| item), vec![&"a", &"b", &"C"]);
    }
}

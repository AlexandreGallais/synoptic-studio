//! The names in a media query list: its media types and its media features' names, read as
//! Stylelint finds them with `@csstools/media-query-list-parser` (`findMediaFeatureNames`,
//! `parseMediaQuery` of Stylelint 17.16.0, REF-STYLELINT-SOURCE) by a simpler reading of ours:
//! the identifiers of each parenthesised condition before its `:` or beside a range operator, or
//! alone; the identifier opening a query, after `not` or `only`. Sass variables (`$a`,
//! `ns.$a`) and functions' arguments are left out.

/// A name and where it is in the parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Named {
    /// The name as written.
    pub name: String,
    /// Its first byte.
    pub start: usize,
}

/// A token of a media query list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Token {
    /// An identifier.
    Ident,
    /// A `$` or `.` of a Sass variable.
    Sass,
    /// `<`, `>` or `=`.
    Range,
    /// `:`.
    Colon,
    /// `,` at the top.
    Comma,
    /// `(` opening a block.
    Open,
    /// `)` closing it.
    Close,
    /// A function's name and `(`: what it holds is skipped.
    Function,
    /// Anything else: numbers, strings, other characters.
    Other,
}

/// Whether a byte may continue an identifier.
fn is_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte >= 0x80 || byte == b'\\'
}

/// The tokens of a media query list, spaces and comments left out, with their ranges.
fn tokens(params: &str) -> Vec<(Token, usize, usize)> {
    let bytes = params.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        let byte = bytes[at];
        let start = at;
        let token = match byte {
            b' ' | b'\t' | b'\n' | b'\r' | 0x0c => {
                at += 1;
                continue;
            }
            b'/' if bytes.get(at + 1) == Some(&b'*') => {
                at = params[at + 2..]
                    .find("*/")
                    .map_or(bytes.len(), |end| at + 2 + end + 2);
                continue;
            }
            b'"' | b'\'' => {
                at += 1;
                while at < bytes.len() && bytes[at] != byte {
                    at += 1 + usize::from(bytes[at] == b'\\');
                }
                at += 1;
                Token::Other
            }
            b'(' => {
                at += 1;
                Token::Open
            }
            b')' => {
                at += 1;
                Token::Close
            }
            b',' => {
                at += 1;
                Token::Comma
            }
            b':' => {
                at += 1;
                Token::Colon
            }
            b'<' | b'>' | b'=' => {
                at += 1;
                Token::Range
            }
            b'$' | b'.'
                if bytes.get(at + 1).is_some_and(|next| {
                    *next == b'$' || next.is_ascii_alphabetic() || *next == b'-' || *next == b'_'
                }) =>
            {
                at += 1;
                Token::Sass
            }
            _ if byte.is_ascii_alphabetic()
                || byte == b'-'
                || byte == b'_'
                || byte >= 0x80
                || byte == b'\\' =>
            {
                let digit_start = byte == b'-' && bytes.get(at + 1).is_some_and(u8::is_ascii_digit);
                while at < bytes.len() && is_name_byte(bytes[at]) {
                    at += 1 + usize::from(bytes[at] == b'\\');
                }
                if digit_start {
                    Token::Other
                } else if bytes.get(at) == Some(&b'(') {
                    at += 1;
                    let mut depth = 1;
                    while at < bytes.len() && depth > 0 {
                        match bytes[at] {
                            b'(' => depth += 1,
                            b')' => depth -= 1,
                            _ => {}
                        }
                        at += 1;
                    }
                    Token::Function
                } else {
                    Token::Ident
                }
            }
            _ => {
                at += 1;
                while at < bytes.len()
                    && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'.' || bytes[at] == b'%')
                {
                    at += 1;
                }
                Token::Other
            }
        };
        found.push((token, start, at.min(bytes.len())));
    }
    found
}

/// The media features' names of a media query list, in the order written.
#[must_use]
pub fn feature_names(params: &str) -> Vec<Named> {
    let tokens = tokens(params);
    let mut found = Vec::new();
    // Each block's own tokens, its nested blocks left out.
    let mut stack: Vec<Vec<(Token, usize, usize)>> = Vec::new();
    for &(token, start, end) in &tokens {
        match token {
            Token::Open => stack.push(Vec::new()),
            Token::Close => {
                if let Some(block) = stack.pop() {
                    found.extend(block_features(params, &block));
                    if let Some(outer) = stack.last_mut() {
                        outer.push((Token::Other, start, end));
                    }
                }
            }
            _ => {
                if let Some(block) = stack.last_mut() {
                    block.push((token, start, end));
                }
            }
        }
    }
    found.sort_by_key(|named| named.start);
    found
}

/// The features a block's own tokens name.
fn block_features(params: &str, block: &[(Token, usize, usize)]) -> Vec<Named> {
    let relevant: Vec<(Token, usize, usize)> = block
        .iter()
        .enumerate()
        .filter(|(at, (token, _, _))| {
            let after_sass = at
                .checked_sub(1)
                .is_some_and(|before| block[before].0 == Token::Sass);
            let before_sass = block.get(at + 1).is_some_and(|next| next.0 == Token::Sass);
            *token != Token::Sass && !(*token == Token::Ident && (after_sass || before_sass))
        })
        .map(|(_, each)| *each)
        .collect();
    let named = |start: usize, end: usize| Named {
        name: params[start..end].to_owned(),
        start,
    };
    if let [(Token::Ident, start, end)] = relevant.as_slice() {
        return vec![named(*start, *end)];
    }
    relevant
        .iter()
        .enumerate()
        .filter(|(at, (token, _, _))| {
            let next = relevant.get(at + 1).map(|each| each.0);
            let previous = at.checked_sub(1).map(|before| relevant[before].0);
            *token == Token::Ident
                && ((*at == 0 && next == Some(Token::Colon))
                    || next == Some(Token::Range)
                    || previous == Some(Token::Range))
        })
        .map(|(_, (_, start, end))| named(*start, *end))
        .collect()
}

/// The media type of each query of a media query list that has one.
#[must_use]
pub fn media_types(params: &str) -> Vec<Named> {
    let tokens = tokens(params);
    let mut found = Vec::new();
    for query in tokens.split(|(token, _, _)| *token == Token::Comma) {
        let mut words = query.iter();
        let mut first = words.next();
        if let Some((Token::Ident, start, end)) = first
            && matches!(params[*start..*end].to_lowercase().as_str(), "not" | "only")
        {
            first = words.next();
        }
        if let Some((Token::Ident, start, end)) = first
            && !params[*start..*end].eq_ignore_ascii_case("and")
        {
            found.push(Named {
                name: params[*start..*end].to_owned(),
                start: *start,
            });
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::{Named, feature_names, media_types};

    /// The names, without their places.
    fn names(found: &[Named]) -> Vec<&str> {
        found.iter().map(|each| each.name.as_str()).collect()
    }

    #[test]
    fn finds_the_features_and_types_of_a_media_query_list() {
        assert_eq!(
            names(&feature_names(
                "screen and (min-width: 1px), not print and (400px <= width) and (hover)"
            )),
            ["min-width", "width", "hover"]
        );
        assert_eq!(
            names(&feature_names("(not (color)) and ($a: 1) and (ns.$b > 2)")),
            ["color"]
        );
        assert_eq!(
            names(&feature_names(
                "(width: calc(1px + a)) /* (b: 1) */ and ('(c: 1)')"
            )),
            ["width"]
        );
        assert_eq!(
            names(&media_types(
                "only screen and (color), not tv, (hover), print"
            )),
            ["screen", "tv", "print"]
        );
        // `(`, `-1px` from byte 1, a space, `<`, a space: `width` from byte 8.
        assert_eq!(feature_names("(-1px < width)")[0].start, 8);
    }
}

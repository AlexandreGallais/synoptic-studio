//! Base64 decoded as JavaScript's `atob` decodes it, for the 1Password rule: the Infra standard's
//! forgiving-base64 decode (`REF-WHATWG-FORGIVING-BASE64`, which the HTML standard's `atob` runs),
//! over RFC 4648's alphabet (`REF-RFC-4648`). The tests read their fixtures through it too.

/// The value of a character of the base64 alphabet, `None` for any other.
fn value(character: u8) -> Option<u32> {
    match character {
        b'A'..=b'Z' => Some(u32::from(character - b'A')),
        b'a'..=b'z' => Some(u32::from(character - b'a') + 26),
        b'0'..=b'9' => Some(u32::from(character - b'0') + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// The bytes of a base64 text, `None` where forgiving-base64 decode fails: ASCII white space is
/// removed; when the length then divides by 4, one or two final `=` are removed; a length leaving
/// 1 over 4, or a character outside the alphabet, fails; the bits of a last partial group that
/// make no byte are dropped.
#[must_use]
pub fn decode(text: &str) -> Option<Vec<u8>> {
    let mut data: Vec<u8> = text
        .bytes()
        .filter(|byte| !matches!(byte, b'\t' | b'\n' | b'\x0C' | b'\r' | b' '))
        .collect();

    if data.len().is_multiple_of(4) {
        if data.ends_with(b"==") {
            data.truncate(data.len() - 2);
        } else if data.ends_with(b"=") {
            data.truncate(data.len() - 1);
        }
    }
    if data.len() % 4 == 1 || !text.is_ascii() {
        return None;
    }
    let mut output = Vec::with_capacity(data.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0;

    for character in data {
        buffer = (buffer << 6) | value(character)?;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push(u8::try_from((buffer >> bits) & 0xff).unwrap_or_default());
        }
    }
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_4648s_vectors_decode() {
        // RFC 4648, section 10.
        assert_eq!(decode("").as_deref(), Some(&b""[..]));
        assert_eq!(decode("Zg==").as_deref(), Some(&b"f"[..]));
        assert_eq!(decode("Zm8=").as_deref(), Some(&b"fo"[..]));
        assert_eq!(decode("Zm9v").as_deref(), Some(&b"foo"[..]));
        assert_eq!(decode("Zm9vYmFy").as_deref(), Some(&b"foobar"[..]));
    }

    #[test]
    fn decoding_forgives_as_the_infra_standard_says() {
        // Missing padding and white space are accepted; "YQ" and "YR" both give "a", the last
        // four bits dropped.
        assert_eq!(decode("Zg").as_deref(), Some(&b"f"[..]));
        assert_eq!(decode(" Zm 9v\n").as_deref(), Some(&b"foo"[..]));
        assert_eq!(decode("YQ"), decode("YR"));
    }

    #[test]
    fn what_forgiving_base64_refuses_fails() {
        // One character over a group of four; `=` left inside; a character outside the alphabet;
        // "Zg===", five characters: no `=` is removed since 5 does not divide by 4, and 5 leaves
        // 1 over 4; a character outside ASCII.
        assert!(decode("Z").is_none());
        assert!(decode("Zg=a").is_none());
        assert!(decode("Zm-v").is_none());
        assert!(decode("Zg===").is_none());
        assert!(decode("Zé==").is_none());
    }
}

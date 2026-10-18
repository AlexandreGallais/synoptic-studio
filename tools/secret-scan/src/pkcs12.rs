//! The GCP rule's check of a `.p12` file, ported from `@secretlint/secretlint-rule-preset-recommend`
//! 13.0.7 (`verifyPkcs12Mac`): a PKCS #12 file (RFC 7292, `REF-RFC-7292`) whose MAC verifies
//! under a password. Only enough of the DER is read to find the `MacData` and the `authSafe`'s
//! content; the MAC key is derived as RFC 7292's appendix B.2 says, with the ID byte 3, and the
//! HMAC recomputed. Any structure the rule does not read, an unknown digest, or an iteration count
//! outside 1 to 1,000,000, is no match.

use crate::sha::Hash;

/// The tags read.
const INTEGER: u8 = 0x02;
/// An OCTET STRING.
const OCTET_STRING: u8 = 0x04;
/// A NULL.
const NULL: u8 = 0x05;
/// An OBJECT IDENTIFIER.
const OID: u8 = 0x06;
/// A SEQUENCE.
const SEQUENCE: u8 = 0x30;
/// The `[0] EXPLICIT` of a `ContentInfo`.
const CONTEXT_0: u8 = 0xa0;

/// The OID `data` (1.2.840.113549.1.7.1), its content bytes.
const OID_DATA: [u8; 9] = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x01];

/// The digests a MAC may use, by their OID's content bytes (RFC 6234, section 1).
const MAC_HASHES: [(&[u8], Hash); 4] = [
    (&[0x2b, 0x0e, 0x03, 0x02, 0x1a], Hash::Sha1),
    (
        &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01],
        Hash::Sha256,
    ),
    (
        &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02],
        Hash::Sha384,
    ),
    (
        &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03],
        Hash::Sha512,
    ),
];

/// The most iterations the rule computes, so that a crafted file cannot hold the scan.
const MAX_ITERATIONS: u64 = 1_000_000;

/// One TLV of DER: its tag, its content and where it ends in the buffer read.
struct Tlv<'a> {
    /// Its tag.
    tag: u8,
    /// Its content.
    content: &'a [u8],
    /// The offset after it.
    end: usize,
}

/// What the MAC check needs of a file.
struct MacData<'a> {
    /// The authSafe's content, the text the MAC is over.
    auth_safe: &'a [u8],
    /// The digest.
    hash: Hash,
    /// The salt.
    salt: &'a [u8],
    /// The MAC to compare.
    digest: &'a [u8],
    /// The iteration count.
    iterations: u64,
}

/// The TLV at `offset`: a short length, or a long one of 1 to 4 bytes.
fn read(buffer: &[u8], offset: usize) -> Option<Tlv<'_>> {
    let tag = *buffer.get(offset)?;
    let first = *buffer.get(offset + 1)?;
    let (length, start) = if first < 0x80 {
        (usize::from(first), offset + 2)
    } else {
        let count = usize::from(first & 0x7f);

        if count == 0 || count > 4 {
            return None;
        }
        let bytes = buffer.get(offset + 2..offset + 2 + count)?;

        (
            bytes
                .iter()
                .fold(0, |length, byte| length * 256 + usize::from(*byte)),
            offset + 2 + count,
        )
    };
    let end = start.checked_add(length)?;

    Some(Tlv {
        tag,
        content: buffer.get(start..end)?,
        end,
    })
}

/// The TLV at `offset`, if its tag is `tag`.
fn expect(buffer: &[u8], offset: usize, tag: u8) -> Option<Tlv<'_>> {
    read(buffer, offset).filter(|tlv| tlv.tag == tag)
}

/// An INTEGER's value, big-endian, `None` past 64 bits.
fn integer(tlv: &Tlv<'_>) -> Option<u64> {
    if tlv.tag != INTEGER {
        return None;
    }
    tlv.content.iter().try_fold(0u64, |value, byte| {
        value.checked_mul(256).map(|value| value + u64::from(*byte))
    })
}

/// The parts of `PFX ::= SEQUENCE { version INTEGER (3), authSafe ContentInfo, macData MacData }`
/// the check reads, the `authSafe` a `data` `ContentInfo`.
fn parse(buffer: &[u8]) -> Option<MacData<'_>> {
    let pfx = expect(buffer, 0, SEQUENCE)?;
    let version = read(pfx.content, 0)?;

    if integer(&version)? != 3 {
        return None;
    }
    let info = expect(pfx.content, version.end, SEQUENCE)?;
    let content_type = expect(info.content, 0, OID)?;

    if content_type.content != OID_DATA {
        return None;
    }
    let explicit = expect(info.content, content_type.end, CONTEXT_0)?;
    let auth_safe = expect(explicit.content, 0, OCTET_STRING)?;
    let mac_data = expect(pfx.content, info.end, SEQUENCE)?;
    let digest_info = expect(mac_data.content, 0, SEQUENCE)?;
    let algorithm = expect(digest_info.content, 0, SEQUENCE)?;
    let oid = expect(algorithm.content, 0, OID)?;
    let hash = MAC_HASHES
        .iter()
        .find(|(bytes, _)| *bytes == oid.content)
        .map(|(_, hash)| *hash)?;

    if oid.end < algorithm.content.len() && read(algorithm.content, oid.end)?.tag != NULL {
        return None;
    }
    let digest = expect(digest_info.content, algorithm.end, OCTET_STRING)?;
    let salt = expect(mac_data.content, digest_info.end, OCTET_STRING)?;
    let iterations = if salt.end < mac_data.content.len() {
        integer(&read(mac_data.content, salt.end)?)?
    } else {
        1
    };

    (1..=MAX_ITERATIONS)
        .contains(&iterations)
        .then_some(MacData {
            auth_safe: auth_safe.content,
            hash,
            salt: salt.content,
            digest: digest.content,
            iterations,
        })
}

/// A string repeated to a whole number of blocks, its last copy cut; empty stays empty
/// (RFC 7292, appendix B.2, steps 2 and 3).
fn repeated(text: &[u8], block: usize) -> Vec<u8> {
    if text.is_empty() {
        return Vec::new();
    }
    let length = text.len().div_ceil(block) * block;

    text.iter().copied().cycle().take(length).collect()
}

/// A password as a `BMPString` with its two-byte NUL terminator, the empty password empty
/// (RFC 7292, appendix B.1).
fn bmp_string(password: &str) -> Vec<u8> {
    if password.is_empty() {
        return Vec::new();
    }
    let mut bytes: Vec<u8> = password.encode_utf16().flat_map(u16::to_be_bytes).collect();

    bytes.extend_from_slice(&[0, 0]);
    bytes
}

/// The MAC key (RFC 7292, appendix B.2 with ID 3): `H^r(D || S || P)`, one block `A_1` since a
/// MAC key is never longer than the digest.
fn mac_key(hash: Hash, password: &str, salt: &[u8], iterations: u64) -> Vec<u8> {
    let block = hash.block_len();
    let mut input = vec![3; block];

    input.extend(repeated(salt, block));
    input.extend(repeated(&bmp_string(password), block));
    let mut key = hash.digest(&input);

    for _ in 1..iterations {
        key = hash.digest(&key);
    }
    key
}

/// Whether the file is a PKCS #12 file whose MAC verifies under the password.
#[must_use]
pub fn verifies(file: &[u8], password: &str) -> bool {
    parse(file).is_some_and(|mac| {
        let key = mac_key(mac.hash, password, mac.salt, mac.iterations);

        mac.hash.hmac(&key, mac.auth_safe) == mac.digest
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_password_is_a_bmp_string_with_its_terminator() {
        // RFC 7292, appendix B.1: "Beavis" is 14 bytes.
        assert_eq!(
            bmp_string("Beavis"),
            [
                0x00, 0x42, 0x00, 0x65, 0x00, 0x61, 0x00, 0x76, 0x00, 0x69, 0x00, 0x73, 0x00, 0x00
            ]
        );
        assert_eq!(bmp_string(""), b"");
    }

    #[test]
    fn a_string_is_repeated_to_whole_blocks() {
        // 3 bytes to a block of 4: one copy and the first byte again; 0 bytes stay 0.
        assert_eq!(repeated(b"abc", 4), b"abca");
        assert_eq!(repeated(b"abcd", 4), b"abcd");
        assert_eq!(repeated(b"", 4), b"");
    }

    #[test]
    fn lengths_short_and_long_are_read() {
        // 04 03 "abc": short; 04 81 03 "abc": one length byte; 04 85 …: five, refused; 04 80:
        // none, refused; 04 05 "abc": past the buffer.
        assert_eq!(
            read(b"\x04\x03abc", 0).map(|tlv| tlv.content),
            Some(&b"abc"[..])
        );
        assert_eq!(read(b"\x04\x81\x03abc", 0).map(|tlv| tlv.end), Some(6));
        assert!(read(b"\x04\x85\x00\x00\x00\x00\x03abc", 0).is_none());
        assert!(read(b"\x04\x80abc", 0).is_none());
        assert!(read(b"\x04\x05abc", 0).is_none());
        assert!(read(b"\x04", 0).is_none());
    }

    #[test]
    fn an_integer_is_read_big_endian() {
        // 02 02 01 00 is 256; a tag other than INTEGER is none; nine bytes overflow 64 bits.
        let tlv = read(b"\x02\x02\x01\x00", 0).expect("an integer");

        assert_eq!(integer(&tlv), Some(256));
        assert!(integer(&read(b"\x04\x01\x01", 0).expect("a string")).is_none());
        assert!(
            integer(&read(b"\x02\x09\x01\x00\x00\x00\x00\x00\x00\x00\x00", 0).expect("long"))
                .is_none()
        );
    }

    #[test]
    fn what_is_no_pkcs12_file_never_verifies() {
        assert!(!verifies(b"", "notasecret"));
        assert!(!verifies(b"\x30\x03\x02\x01\x02", "notasecret"));
        assert!(!verifies(b"not a p12 at all", "notasecret"));
    }
}

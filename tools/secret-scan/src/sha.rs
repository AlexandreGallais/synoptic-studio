//! SHA-1, SHA-256, SHA-384 and SHA-512, and HMAC over each, for the GCP rule's PKCS #12 check
//! alone. Written from their standards rather than taken from a crate (EN-110, T1): SHA-1 from
//! RFC 3174 (`REF-RFC-3174`), the SHA-2 functions from RFC 6234 (`REF-RFC-6234`), HMAC from
//! RFC 2104 (`REF-RFC-2104`); tested on RFC 6234's own vectors. A message is whole bytes, so the
//! padding appends the byte 0x80, zeros and the length in bits.

/// A hash function of the four.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hash {
    /// SHA-1, 20 bytes out of 64-byte blocks.
    Sha1,
    /// SHA-256, 32 bytes out of 64-byte blocks.
    Sha256,
    /// SHA-384, 48 bytes out of 128-byte blocks.
    Sha384,
    /// SHA-512, 64 bytes out of 128-byte blocks.
    Sha512,
}

/// SHA-256's constants, RFC 6234 section 5.1.
const K256: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// SHA-384's and SHA-512's constants, RFC 6234 section 5.2.
const K512: [u64; 80] = [
    0x428a_2f98_d728_ae22,
    0x7137_4491_23ef_65cd,
    0xb5c0_fbcf_ec4d_3b2f,
    0xe9b5_dba5_8189_dbbc,
    0x3956_c25b_f348_b538,
    0x59f1_11f1_b605_d019,
    0x923f_82a4_af19_4f9b,
    0xab1c_5ed5_da6d_8118,
    0xd807_aa98_a303_0242,
    0x1283_5b01_4570_6fbe,
    0x2431_85be_4ee4_b28c,
    0x550c_7dc3_d5ff_b4e2,
    0x72be_5d74_f27b_896f,
    0x80de_b1fe_3b16_96b1,
    0x9bdc_06a7_25c7_1235,
    0xc19b_f174_cf69_2694,
    0xe49b_69c1_9ef1_4ad2,
    0xefbe_4786_384f_25e3,
    0x0fc1_9dc6_8b8c_d5b5,
    0x240c_a1cc_77ac_9c65,
    0x2de9_2c6f_592b_0275,
    0x4a74_84aa_6ea6_e483,
    0x5cb0_a9dc_bd41_fbd4,
    0x76f9_88da_8311_53b5,
    0x983e_5152_ee66_dfab,
    0xa831_c66d_2db4_3210,
    0xb003_27c8_98fb_213f,
    0xbf59_7fc7_beef_0ee4,
    0xc6e0_0bf3_3da8_8fc2,
    0xd5a7_9147_930a_a725,
    0x06ca_6351_e003_826f,
    0x1429_2967_0a0e_6e70,
    0x27b7_0a85_46d2_2ffc,
    0x2e1b_2138_5c26_c926,
    0x4d2c_6dfc_5ac4_2aed,
    0x5338_0d13_9d95_b3df,
    0x650a_7354_8baf_63de,
    0x766a_0abb_3c77_b2a8,
    0x81c2_c92e_47ed_aee6,
    0x9272_2c85_1482_353b,
    0xa2bf_e8a1_4cf1_0364,
    0xa81a_664b_bc42_3001,
    0xc24b_8b70_d0f8_9791,
    0xc76c_51a3_0654_be30,
    0xd192_e819_d6ef_5218,
    0xd699_0624_5565_a910,
    0xf40e_3585_5771_202a,
    0x106a_a070_32bb_d1b8,
    0x19a4_c116_b8d2_d0c8,
    0x1e37_6c08_5141_ab53,
    0x2748_774c_df8e_eb99,
    0x34b0_bcb5_e19b_48a8,
    0x391c_0cb3_c5c9_5a63,
    0x4ed8_aa4a_e341_8acb,
    0x5b9c_ca4f_7763_e373,
    0x682e_6ff3_d6b2_b8a3,
    0x748f_82ee_5def_b2fc,
    0x78a5_636f_4317_2f60,
    0x84c8_7814_a1f0_ab72,
    0x8cc7_0208_1a64_39ec,
    0x90be_fffa_2363_1e28,
    0xa450_6ceb_de82_bde9,
    0xbef9_a3f7_b2c6_7915,
    0xc671_78f2_e372_532b,
    0xca27_3ece_ea26_619c,
    0xd186_b8c7_21c0_c207,
    0xeada_7dd6_cde0_eb1e,
    0xf57d_4f7f_ee6e_d178,
    0x06f0_67aa_7217_6fba,
    0x0a63_7dc5_a2c8_98a6,
    0x113f_9804_bef9_0dae,
    0x1b71_0b35_131c_471b,
    0x28db_77f5_2304_7d84,
    0x32ca_ab7b_40c7_2493,
    0x3c9e_be0a_15c9_bebc,
    0x431d_67c4_9c10_0d4c,
    0x4cc5_d4be_cb3e_42b6,
    0x597f_299c_fc65_7e2a,
    0x5fcb_6fab_3ad6_faec,
    0x6c44_198c_4a47_5817,
];

impl Hash {
    /// The length of its digest in bytes, `u / 8` of RFC 7292's table.
    #[must_use]
    pub const fn output_len(self) -> usize {
        match self {
            Self::Sha1 => 20,
            Self::Sha256 => 32,
            Self::Sha384 => 48,
            Self::Sha512 => 64,
        }
    }

    /// The length of its block in bytes, `v / 8` of RFC 7292's table.
    #[must_use]
    pub const fn block_len(self) -> usize {
        match self {
            Self::Sha1 | Self::Sha256 => 64,
            Self::Sha384 | Self::Sha512 => 128,
        }
    }

    /// The digest of a message.
    #[must_use]
    pub fn digest(self, message: &[u8]) -> Vec<u8> {
        match self {
            Self::Sha1 => sha1(message).to_vec(),
            Self::Sha256 => sha256(message).to_vec(),
            Self::Sha384 => sha512_family(message, SHA384_INITIAL)[..48].to_vec(),
            Self::Sha512 => sha512_family(message, SHA512_INITIAL).to_vec(),
        }
    }

    /// HMAC of a text under a key (RFC 2104, section 2): a key longer than a block is hashed
    /// first; `H(K ⊕ opad, H(K ⊕ ipad, text))`, the key padded with zeros to a block.
    #[must_use]
    pub fn hmac(self, key: &[u8], text: &[u8]) -> Vec<u8> {
        let block = self.block_len();
        let mut padded = if key.len() > block {
            self.digest(key)
        } else {
            key.to_vec()
        };

        padded.resize(block, 0);
        let mut inner: Vec<u8> = padded.iter().map(|byte| byte ^ 0x36).collect();
        inner.extend_from_slice(text);
        let mut outer: Vec<u8> = padded.iter().map(|byte| byte ^ 0x5c).collect();
        outer.extend_from_slice(&self.digest(&inner));
        self.digest(&outer)
    }
}

/// The message padded to whole blocks: 0x80, zeros, then its length in bits on `length_len`
/// bytes, big-endian (RFC 6234 section 4, RFC 3174 section 4).
fn padded(message: &[u8], block: usize, length_len: usize) -> Vec<u8> {
    let bits = (message.len() as u128) * 8;
    let mut padded = message.to_vec();

    padded.push(0x80);
    while padded.len() % block != block - length_len {
        padded.push(0);
    }
    padded.extend_from_slice(&bits.to_be_bytes()[16 - length_len..]);
    padded
}

/// SHA-1, RFC 3174 section 6.1.
#[expect(
    clippy::many_single_char_names,
    reason = "the names of the standard's steps: a to h, w, h, t"
)]
fn sha1(message: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [
        0x6745_2301,
        0xEFCD_AB89,
        0x98BA_DCFE,
        0x1032_5476,
        0xC3D2_E1F0,
    ];

    for block in padded(message, 64, 8).as_chunks::<64>().0 {
        let mut w = [0u32; 80];

        for (t, word) in block.as_chunks::<4>().0.iter().enumerate() {
            w[t] = u32::from_be_bytes(*word);
        }
        for t in 16..80 {
            w[t] = (w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]).rotate_left(1);
        }
        let [mut a, mut b, mut c, mut d, mut e] = h;

        for (t, word) in w.iter().enumerate() {
            let (f, k) = match t {
                0..=19 => ((b & c) | (!b & d), 0x5A82_7999),
                20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
                _ => (b ^ c ^ d, 0xCA62_C1D6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(*word)
                .wrapping_add(k);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        for (word, added) in h.iter_mut().zip([a, b, c, d, e]) {
            *word = word.wrapping_add(added);
        }
    }
    let mut digest = [0; 20];

    for (bytes, word) in digest.as_chunks_mut::<4>().0.iter_mut().zip(h) {
        bytes.copy_from_slice(&word.to_be_bytes());
    }
    digest
}

/// SHA-256, RFC 6234 sections 6.1 and 6.2.
#[expect(
    clippy::many_single_char_names,
    reason = "the names of the standard's steps: a to h, w, h, t"
)]
fn sha256(message: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09_e667,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];

    for block in padded(message, 64, 8).as_chunks::<64>().0 {
        let mut w = [0u32; 64];

        for (t, word) in block.as_chunks::<4>().0.iter().enumerate() {
            w[t] = u32::from_be_bytes(*word);
        }
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);

            w[t] = s1
                .wrapping_add(w[t - 7])
                .wrapping_add(s0)
                .wrapping_add(w[t - 16]);
        }
        let mut v = h;

        for t in 0..64 {
            let [a, b, c, d, e, f, g, hh] = v;
            let big1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(big1)
                .wrapping_add(ch)
                .wrapping_add(K256[t])
                .wrapping_add(w[t]);
            let big0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = big0.wrapping_add(maj);

            v = [t1.wrapping_add(t2), a, b, c, d.wrapping_add(t1), e, f, g];
        }
        for (word, added) in h.iter_mut().zip(v) {
            *word = word.wrapping_add(added);
        }
    }
    let mut digest = [0; 32];

    for (bytes, word) in digest.as_chunks_mut::<4>().0.iter_mut().zip(h) {
        bytes.copy_from_slice(&word.to_be_bytes());
    }
    digest
}

/// SHA-384's initial hash value, RFC 6234 section 6.3.
const SHA384_INITIAL: [u64; 8] = [
    0xcbbb_9d5d_c105_9ed8,
    0x629a_292a_367c_d507,
    0x9159_015a_3070_dd17,
    0x152f_ecd8_f70e_5939,
    0x6733_2667_ffc0_0b31,
    0x8eb4_4a87_6858_1511,
    0xdb0c_2e0d_64f9_8fa7,
    0x47b5_481d_befa_4fa4,
];

/// SHA-512's initial hash value, RFC 6234 section 6.3.
const SHA512_INITIAL: [u64; 8] = [
    0x6a09_e667_f3bc_c908,
    0xbb67_ae85_84ca_a73b,
    0x3c6e_f372_fe94_f82b,
    0xa54f_f53a_5f1d_36f1,
    0x510e_527f_ade6_82d1,
    0x9b05_688c_2b3e_6c1f,
    0x1f83_d9ab_fb41_bd6b,
    0x5be0_cd19_137e_2179,
];

/// SHA-512 from an initial value, RFC 6234 section 6.4; SHA-384 keeps its first 48 bytes.
#[expect(
    clippy::many_single_char_names,
    reason = "the names of the standard's steps: a to h, w, h, t"
)]
fn sha512_family(message: &[u8], initial: [u64; 8]) -> [u8; 64] {
    let mut h = initial;

    for block in padded(message, 128, 16).as_chunks::<128>().0 {
        let mut w = [0u64; 80];

        for (t, word) in block.as_chunks::<8>().0.iter().enumerate() {
            w[t] = u64::from_be_bytes(*word);
        }
        for t in 16..80 {
            let s0 = w[t - 15].rotate_right(1) ^ w[t - 15].rotate_right(8) ^ (w[t - 15] >> 7);
            let s1 = w[t - 2].rotate_right(19) ^ w[t - 2].rotate_right(61) ^ (w[t - 2] >> 6);

            w[t] = s1
                .wrapping_add(w[t - 7])
                .wrapping_add(s0)
                .wrapping_add(w[t - 16]);
        }
        let mut v = h;

        for t in 0..80 {
            let [a, b, c, d, e, f, g, hh] = v;
            let big1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(big1)
                .wrapping_add(ch)
                .wrapping_add(K512[t])
                .wrapping_add(w[t]);
            let big0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = big0.wrapping_add(maj);

            v = [t1.wrapping_add(t2), a, b, c, d.wrapping_add(t1), e, f, g];
        }
        for (word, added) in h.iter_mut().zip(v) {
            *word = word.wrapping_add(added);
        }
    }
    let mut digest = [0; 64];

    for (bytes, word) in digest.as_chunks_mut::<8>().0.iter_mut().zip(h) {
        bytes.copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use super::*;

    /// A digest in capital hexadecimal, as RFC 6234 writes its results.
    fn hex(bytes: &[u8]) -> String {
        bytes.iter().fold(String::new(), |mut text, byte| {
            let _ = write!(text, "{byte:02X}");
            text
        })
    }

    /// RFC 6234 section 8.5, TEST1 and TEST2: "abc" and the 448-bit (SHA-1, SHA-256) or 896-bit
    /// (SHA-384, SHA-512) messages, each hash's results of its table.
    #[test]
    fn the_standard_messages_give_rfc_6234s_digests() {
        let test2_1 = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let test2_2 = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";

        assert_eq!(
            hex(&Hash::Sha1.digest(b"abc")),
            "A9993E364706816ABA3E25717850C26C9CD0D89D"
        );
        assert_eq!(
            hex(&Hash::Sha1.digest(test2_1)),
            "84983E441C3BD26EBAAE4AA1F95129E5E54670F1"
        );
        assert_eq!(
            hex(&Hash::Sha256.digest(b"abc")),
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
        );
        assert_eq!(
            hex(&Hash::Sha256.digest(test2_1)),
            "248D6A61D20638B8E5C026930C3E6039A33CE45964FF2167F6ECEDD419DB06C1"
        );
        assert_eq!(
            hex(&Hash::Sha384.digest(b"abc")),
            "CB00753F45A35E8BB5A03D699AC65007272C32AB0EDED1631A8B605A43FF5BED8086072BA1E7CC2358BAECA134C825A7"
        );
        assert_eq!(
            hex(&Hash::Sha384.digest(test2_2)),
            "09330C33F71147E83D192FC782CD1B4753111B173B3B05D22FA08086E3B0F712FCC7C71A557E2DB966C3E9FA91746039"
        );
        assert_eq!(
            hex(&Hash::Sha512.digest(b"abc")),
            "DDAF35A193617ABACC417349AE20413112E6FA4E89A97EA20A9EEEE64B55D39A2192992A274FC1A836BA3C23A3FEEBBD454D4423643CE80E2A9AC94FA54CA49F"
        );
        assert_eq!(
            hex(&Hash::Sha512.digest(test2_2)),
            "8E959B75DAE313DA8CF4F72814FC143F8F7779C6EB9F7FA17299AEADB6889018501D289E4900F7E4331B99DEC4B5433AC7D329EEB6DD26545E96E55B874BE909"
        );
    }

    /// RFC 6234 section 8.5, TEST4: "01234567" eight times, repeated ten times, 5,120 bits, an
    /// exact multiple of a 512-bit block, so that the padding takes a block of its own; and the
    /// one-byte messages of test 6.
    #[test]
    fn whole_blocks_and_single_bytes_give_rfc_6234s_digests() {
        let test4 = b"0123456701234567012345670123456701234567012345670123456701234567".repeat(10);

        assert_eq!(
            hex(&Hash::Sha1.digest(&test4)),
            "DEA356A2CDDD90C7A7ECEDC5EBB563934F460452"
        );
        assert_eq!(
            hex(&Hash::Sha256.digest(&test4)),
            "594847328451BDFA85056225462CC1D867D877FB388DF0CE35F25AB5562BFBB5"
        );
        assert_eq!(
            hex(&Hash::Sha1.digest(b"\x5e")),
            "5E6F80A34A9798CAFC6A5DB96CC57BA4C4DB59C2"
        );
        assert_eq!(
            hex(&Hash::Sha256.digest(b"\x19")),
            "68AA2E2EE5DFF96E3355E6C7EE373E3D6A4E17F75F9518D843709C0C9BC3E3D4"
        );
        assert_eq!(
            hex(&Hash::Sha384.digest(b"\xb9")),
            "BC8089A19007C0B14195F4ECC74094FEC64F01F90929282C2FB392881578208AD466828B1C6C283D2722CF0AD1AB6938"
        );
        assert_eq!(
            hex(&Hash::Sha512.digest(b"\xD0")),
            "9992202938E882E73E20F6B69E68A0A7149090423D93C81BAB3F21678D4ACEEEE50E4E8CAFADA4C85A54EA8306826C4AD6E74CECE9631BFA8A549B4AB3FBBA15"
        );
    }

    /// RFC 6234 section 8.5, the HMAC tests 1 and 2: twenty bytes 0x0b over "Hi There", and the
    /// key "Jefe" over "what do ya want for nothing?".
    #[test]
    fn hmac_gives_rfc_6234s_results() {
        let key = [0x0b; 20];

        assert_eq!(
            hex(&Hash::Sha1.hmac(&key, b"Hi There")),
            "B617318655057264E28BC0B6FB378C8EF146BE00"
        );
        assert_eq!(
            hex(&Hash::Sha256.hmac(&key, b"Hi There")),
            "B0344C61D8DB38535CA8AFCEAF0BF12B881DC200C9833DA726E9376C2E32CFF7"
        );
        assert_eq!(
            hex(&Hash::Sha384.hmac(&key, b"Hi There")),
            "AFD03944D84895626B0825F4AB46907F15F9DADBE4101EC682AA034C7CEBC59CFAEA9EA9076EDE7F4AF152E8B2FA9CB6"
        );
        assert_eq!(
            hex(&Hash::Sha512.hmac(&key, b"Hi There")),
            "87AA7CDEA5EF619D4FF0B4241A1D6CB02379F4E2CE4EC2787AD0B30545E17CDEDAA833B7D6B8A702038B274EAEA3F4E4BE9D914EEB61F1702E696C203A126854"
        );
        assert_eq!(
            hex(&Hash::Sha1.hmac(b"Jefe", b"what do ya want for nothing?")),
            "EFFCDF6AE5EB2FA2D27416D5F184DF9C259A7C79"
        );
    }

    /// RFC 6234 section 8.5, HMAC test 6: a key longer than a block, hashed first: 80 bytes 0xaa
    /// for SHA-1, 131 for the others.
    #[test]
    fn hmac_hashes_a_key_longer_than_a_block() {
        let text = b"Test Using Larger Than Block-Size Key - Hash Key First";

        assert_eq!(
            hex(&Hash::Sha1.hmac(&[0xaa; 80], text)),
            "AA4AE5E15272D00E95705637CE8A3B55ED402112"
        );
        assert_eq!(
            hex(&Hash::Sha256.hmac(&[0xaa; 131], text)),
            "60E431591EE0B67F0D8A26AACBF5B77F8E0BC6213728C5140546040F0EE37F54"
        );
        assert_eq!(
            hex(&Hash::Sha512.hmac(&[0xaa; 131], text)),
            "80B24263C7C1A3EBB71493C1DD7BE8B49B46D1F41B4AEEC1121B013783F8F3526B56D037E05F2598BD0FD2215D6A1E5295E64F73F63F0AEC8B915A985D786598"
        );
    }

    #[test]
    fn the_lengths_are_rfc_7292s() {
        // RFC 7292 appendix B.2: u and v in bits, here in bytes.
        assert_eq!((Hash::Sha1.output_len(), Hash::Sha1.block_len()), (20, 64));
        assert_eq!(
            (Hash::Sha256.output_len(), Hash::Sha256.block_len()),
            (32, 64)
        );
        assert_eq!(
            (Hash::Sha384.output_len(), Hash::Sha384.block_len()),
            (48, 128)
        );
        assert_eq!(
            (Hash::Sha512.output_len(), Hash::Sha512.block_len()),
            (64, 128)
        );
    }
}

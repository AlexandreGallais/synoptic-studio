import { ceiling, elementAddress, integerQuotient, modulo } from "../../math";

/** The 64 characters of base 64, each at its 6-bit value: `A`–`Z` 0–25, `a`–`z` 26–51, `0`–`9` 52–61, `+` 62, `/` 63 (RFC 4648, Table 1). */
const BASE64_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/** The bytes of one group: 24 bits. */
const GROUP_BYTES = 3;

/** The character that pads the last group to four (RFC 4648, §4). */
const PAD = "=";

/** 2², the values of 2 bits. */
const TWO_BITS = 4;

/** 2⁴, the values of 4 bits. */
const FOUR_BITS = 16;

/** 2⁶, the values of 6 bits: one character of base 64. */
const SIX_BITS = 64;

/**
 * The four 6-bit values of a group of three bytes, the first byte the most significant
 * (`DERIV-font-embedding`, step 4).
 *
 * Formula: c₁ = ⌊b₀ / 4⌋, c₂ = (b₀ mod 4) · 16 + ⌊b₁ / 16⌋, c₃ = (b₁ mod 16) · 4 + ⌊b₂ / 64⌋,
 * c₄ = b₂ mod 64.
 *
 * @kind format
 * @param first - b₀, from 0 to 255
 * @param second - b₁, from 0 to 255
 * @param third - b₂, from 0 to 255
 * @returns the four values, each from 0 to 63
 */
function bytesToSextets(first: number, second: number, third: number): readonly number[] {
  return [
    integerQuotient(first, TWO_BITS),
    modulo(first, TWO_BITS) * FOUR_BITS + integerQuotient(second, FOUR_BITS),
    modulo(second, FOUR_BITS) * TWO_BITS + integerQuotient(third, SIX_BITS),
    modulo(third, SIX_BITS),
  ];
}

/**
 * The byte at a place, or the zero bits that fill a last group (`DERIV-font-embedding`, step 5).
 *
 * @kind format
 * @param bytes - the bytes encoded
 * @param index - a place, past the end in a last group
 * @returns the byte; 0 past the end
 */
function indexToByte(bytes: Readonly<Uint8Array>, index: number): number {
  return bytes[index] ?? 0;
}

/**
 * The four characters of the group of bytes from a place: a last group of one byte gives two
 * characters and `==`, of two bytes three characters and `=`, its missing bits zero
 * (`DERIV-font-embedding`, step 5).
 *
 * @kind format
 * @param bytes - the bytes encoded
 * @param start - the group's first byte, a multiple of 3 within the bytes
 * @returns four characters
 */
function groupToBase64(bytes: Readonly<Uint8Array>, start: number): string {
  const count = Math.min(GROUP_BYTES, bytes.length - start);
  const values = bytesToSextets(
    indexToByte(bytes, start),
    indexToByte(bytes, start + 1),
    indexToByte(bytes, start + 2),
  );
  const characters = values.slice(0, count + 1).map((value) => BASE64_ALPHABET.charAt(value));

  return characters.join("") + PAD.repeat(GROUP_BYTES - count);
}

/**
 * Bytes written in base 64 (RFC 4648, §4): each group of three bytes as four characters of the
 * alphabet `A`–`Z`, `a`–`z`, `0`–`9`, `+`, `/`, the last group padded with `=`; `n` bytes give
 * `4 · ⌈n / 3⌉` characters (`DERIV-font-embedding`, step 6), none for no byte. Written in
 * ECMAScript alone, without a host's encoder (ADR-0025).
 *
 * @kind format
 * @param bytes - the bytes, such as a font file's
 * @returns their base-64 text
 */
export function bytesToBase64(bytes: Readonly<Uint8Array>): string {
  return Array.from({ length: ceiling(bytes.length / GROUP_BYTES) }, (unused, index) =>
    groupToBase64(bytes, elementAddress(0, GROUP_BYTES, index)),
  ).join("");
}

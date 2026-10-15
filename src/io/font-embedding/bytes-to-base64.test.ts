import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bytesToBase64 } from "./bytes-to-base64";

/** The alphabet of base 64 (RFC 4648, Table 1), so that the test decodes by the source, not by the code. */
const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/**
 * Decodes a base-64 text by its alphabet: each character gives 6 bits, the bits are read 8 at a
 * time, and the padding is dropped (RFC 4648, §4).
 *
 * @param text - a base-64 text
 * @returns the bytes it encodes
 */
function decodedBytes(text: string): readonly number[] {
  const bits = Array.from(text.replaceAll("=", ""), (character) =>
    ALPHABET.indexOf(character).toString(2).padStart(6, "0"),
  ).join("");

  return Array.from({ length: Math.floor(bits.length / 8) }, (unused, index) =>
    Number.parseInt(bits.slice(8 * index, 8 * index + 8), 2),
  );
}

/**
 * The bytes of an ASCII string.
 *
 * @param text - ASCII characters
 * @returns their codes
 */
function asciiBytes(text: string): Uint8Array {
  const encoder = new TextEncoder();

  return encoder.encode(text);
}

describe("bytesToBase64 (REF-RFC-4648, DERIV-font-embedding steps 4 to 6)", () => {
  it.each([
    // RFC 4648, §10. "f" = 102: c₁ = ⌊102 / 4⌋ = 25 (Z), c₂ = 2 · 16 + 0 = 32 (g), then "==".
    { encoded: "", text: "" },
    { encoded: "Zg==", text: "f" },
    // "fo" = 102, 111: Z, m (32 + ⌊111 / 16⌋ = 38), c₃ = 15 · 4 + 0 = 60 (8), then "=".
    { encoded: "Zm8=", text: "fo" },
    // "foo" = 102, 111, 111: Z, m, 9 (60 + ⌊111 / 64⌋ = 61), v (111 mod 64 = 47).
    { encoded: "Zm9v", text: "foo" },
    { encoded: "Zm9vYg==", text: "foob" },
    { encoded: "Zm9vYmE=", text: "fooba" },
    { encoded: "Zm9vYmFy", text: "foobar" },
  ])("[F15.AC2] encodes $text as $encoded", ({ encoded, text }) => {
    expect(bytesToBase64(asciiBytes(text))).toBe(encoded);
  });

  it("uses the last two characters of the alphabet and its first", () => {
    // 0xFB 0xFF 0xBF = 111110 111111 111110 111111: 62, 63, 62, 63 → "+/+/"; three zeros → "AAAA".
    expect(bytesToBase64(Uint8Array.from([0xfb, 0xff, 0xbf]))).toBe("+/+/");
    expect(bytesToBase64(Uint8Array.from([0, 0, 0]))).toBe("AAAA");
    // 255 = 111111 11, padded 110000: 63, 48 → "/w==".
    expect(bytesToBase64(Uint8Array.from([255]))).toBe("/w==");
  });

  it("encodes the bytes of a view, not the whole buffer under it", () => {
    // "xfoox" viewed from 1 to 4 is "foo".
    expect(bytesToBase64(asciiBytes("xfoox").subarray(1, 4))).toBe("Zm9v");
  });

  test.prop({ bytes: fc.uint8Array({ maxLength: 64 }) })(
    "[F15.AC2] writes 4 · ⌈n / 3⌉ characters, which decode to the bytes",
    ({ bytes }) => {
      const encoded = bytesToBase64(bytes);

      expect(encoded).toHaveLength(4 * Math.ceil(bytes.length / 3));
      expect(decodedBytes(encoded)).toEqual([...bytes]);
    },
  );
});

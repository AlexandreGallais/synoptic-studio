import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_BYTE_RGB } from "../../../tests";

import { bytesToHex } from "./bytes-to-hex";
import { hexToBytes } from "./hex-to-bytes";

describe("bytesToHex (REF-CSS-COLOR-4 §16.2.1)", () => {
  it("[F16.AC1] writes each byte in two lower-case digits, a leading zero kept", () => {
    // 255 = ff, 128 = 80, 0 = 00; 10 = 0a, 171 = ab, 205 = CD.
    expect(bytesToHex([255, 128, 0])).toBe("#ff8000");
    expect(bytesToHex([10, 171, 205])).toBe("#0aabcd");
  });

  test.prop({ bytes: ANY_BYTE_RGB })(
    "writes # and six lower-case digits, read back as the same bytes",
    ({ bytes }) => {
      const hex = bytesToHex([bytes.red, bytes.green, bytes.blue]);

      expect(hex).toMatch(/^#[\da-f]{6}$/u);
      expect(hexToBytes(hex)).toEqual(bytes);
    },
  );
});

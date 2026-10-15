import { describe, expect, it } from "vitest";

import { bytesToHex } from "./bytes-to-hex";

describe("bytesToHex (REF-CSS-COLOR-4 §16.2.1)", () => {
  it("[F16.AC1] writes each byte in two lower-case digits, a leading zero kept", () => {
    // 255 = ff, 128 = 80, 0 = 00; 10 = 0a, 171 = ab, 205 = CD.
    expect(bytesToHex([255, 128, 0])).toBe("#ff8000");
    expect(bytesToHex([10, 171, 205])).toBe("#0aabcd");
  });
});

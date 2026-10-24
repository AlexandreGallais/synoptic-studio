import { describe, expect, it } from "vitest";

import { formatByteSize } from "./format-byte-size";

describe("formatByteSize (F40)", () => {
  it.each([
    // Below 1 000, in bytes.
    { bytes: 0, text: "0 B" },
    { bytes: 999, text: "999 B" },
    // 1 000 / 1 000 = 1.0, its decimal of 0 left out.
    { bytes: 1000, text: "1 kB" },
    // 1 248 / 1 000 = 1.248, to one decimal 1.2.
    { bytes: 1248, text: "1.2 kB" },
    // 1 250 / 1 000 = 1.25, a half rounded up to 1.3.
    { bytes: 1250, text: "1.3 kB" },
    // 999 949 / 1 000 = 999.949, 999.9; 999 950 rounds to 1 000.0 kB, shown as 1 MB.
    { bytes: 999_949, text: "999.9 kB" },
    { bytes: 999_950, text: "1 MB" },
    // 100 000 000 / 1 000 000 = 100.
    { bytes: 100_000_000, text: "100 MB" },
    // 2 500 000 000 / 1 000 000 000 = 2.5.
    { bytes: 2_500_000_000, text: "2.5 GB" },
    // GB the largest unit: 2 000 000 000 000 / 10⁹ = 2 000.
    { bytes: 2_000_000_000_000, text: "2000 GB" },
  ])("writes $bytes bytes as $text", ({ bytes, text }) => {
    expect(formatByteSize(bytes)).toBe(text);
  });
});

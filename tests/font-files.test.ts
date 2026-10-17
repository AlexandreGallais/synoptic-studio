import { describe, expect, it } from "vitest";

import { interRegularFile, partsDigest } from "./font-fixtures";
import { sharedSansTables } from "./inter-face-fixtures";

describe("the test font files, read once per run (EN-089)", () => {
  it("reads Inter Regular 4.1 whole: 411 640 bytes, its SHA-256 that of the release", () => {
    // REF-INTER-REGULAR-FILE: 411 640 bytes, SHA-256 40d692fc…be0c82, read with shasum -a 256.
    const file = interRegularFile();

    expect(file.length).toBe(411_640);

    expect(partsDigest([file])).toBe(
      "40d692fce188e4471e2b3cba937be967878f631ad3ebbbdcd587687c7ebe0c82",
    );
  });

  it("gives a fresh copy at each call: a test that writes into it leaves the next one whole", () => {
    const written = interRegularFile();

    written.fill(0);

    expect(interRegularFile()[0]).toBe(0);
    expect(interRegularFile()[1]).toBe(1);
  });

  it("gives Inter's tables frozen: a test cannot change them for the next", () => {
    const [regular] = sharedSansTables();
    const glyf = regular?.["glyf"];

    expect(Object.isFrozen(regular)).toBe(true);
    expect(Array.isArray(glyf) && Object.isFrozen(glyf)).toBe(true);
    expect(sharedSansTables()[0]).toBe(regular);
  });
});

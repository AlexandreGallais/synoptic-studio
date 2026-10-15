import { describe, expect, it } from "vitest";

import { leftOutReasonOf } from "./left-out-reason-of";

/** The bytes of a file, their content unread here. */
const BYTES = Uint8Array.from([0, 1, 0, 0]);

describe("leftOutReasonOf (DERIV-font-embedding, step 3)", () => {
  it("[F15.AC2] leaves out a face whose file is not held, as Inter's", () => {
    expect(leftOutReasonOf(undefined)).toBe("no-file");
  });

  it("[F15.AC2] leaves out a file whose fsType does not permit embedding it", () => {
    // fsType 2, restricted licence: isEmbeddable false when read.
    expect(leftOutReasonOf({ bytes: BYTES, isEmbeddable: false, version: "" })).toBe(
      "not-permitted",
    );
  });

  it("[F15.AC2] keeps a file that may be embedded", () => {
    expect(leftOutReasonOf({ bytes: BYTES, isEmbeddable: true, version: "" })).toBeUndefined();
  });
});

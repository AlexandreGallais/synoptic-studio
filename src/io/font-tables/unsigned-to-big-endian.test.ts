import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { unsignedToBigEndian } from "./unsigned-to-big-endian";

describe("unsignedToBigEndian (REF-OT-OTFF)", () => {
  it.each([
    // 0x0102 = 1 · 256 + 2.
    { bytes: [1, 2], size: 2, value: 0x01_02 },
    // 0x01020304 = 1 · 256³ + 2 · 256² + 3 · 256 + 4.
    { bytes: [1, 2, 3, 4], size: 4, value: 0x01_02_03_04 },
    // 2³² − 1, the largest uint32: every byte 255.
    { bytes: [255, 255, 255, 255], size: 4, value: 0xff_ff_ff_ff },
    // 52 = 0x34, a table offset of a face in a collection: three zero bytes first.
    { bytes: [0, 0, 0, 52], size: 4, value: 52 },
    { bytes: [0, 0], size: 2, value: 0 },
  ])("writes $value on $size bytes as $bytes", ({ bytes, size, value }) => {
    expect(unsignedToBigEndian(value, size)).toEqual(bytes);
  });

  test.prop({ value: fc.integer({ max: 0xff_ff_ff_ff, min: 0 }) })(
    "gives back the value a DataView reads, big-endian",
    ({ value }) => {
      const view = new DataView(Uint8Array.from(unsignedToBigEndian(value, 4)).buffer);

      expect(view.getUint32(0)).toBe(value);
    },
  );
});

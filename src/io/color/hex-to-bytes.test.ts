import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_BYTE_RGB } from "../../../tests";

import { bytesToHex } from "./bytes-to-hex";
import { hexToBytes } from "./hex-to-bytes";

describe("hexToBytes (REF-CSS-COLOR-4 §5.2)", () => {
  it.each([
    // 0x33 = 3 · 16 + 3 = 51, 0x66 = 102, 0x99 = 153.
    { bytes: { blue: 153, green: 102, red: 51 }, color: "#336699" },
    // 0xff = 255, 0x80 = 128, 0x01 = 1: each pair read at its own place.
    { bytes: { blue: 1, green: 128, red: 255 }, color: "#ff8001" },
  ])("[F16.AC1] reads $color as its bytes", ({ bytes, color }) => {
    expect(hexToBytes(color)).toEqual(bytes);
  });

  test.prop({ bytes: ANY_BYTE_RGB })("reads back the bytes bytesToHex wrote", ({ bytes }) => {
    expect(hexToBytes(bytesToHex([bytes.red, bytes.green, bytes.blue]))).toEqual(bytes);
  });
});

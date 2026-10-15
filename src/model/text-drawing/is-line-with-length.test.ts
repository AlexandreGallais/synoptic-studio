import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isLineWithLength } from "./is-line-with-length";

/** 'Hello' in Arial 16, aligned left. */
const HELLO_LINE = { content: "Hello", left: 0, width: 36.4609375, x: 0, y: 14.484375 };

describe("isLineWithLength (DERIV-text-box step 10)", () => {
  it.each([
    { isExpected: true, width: 36.4609375 },
    { isExpected: false, width: 0 },
    { isExpected: false, width: -0 },
    { isExpected: false, width: -8 },
  ])("[F14.AC4] says a line $width wide has a length: $isExpected", ({ isExpected, width }) => {
    expect(isLineWithLength({ ...HELLO_LINE, width })).toBe(isExpected);
  });

  test.prop({ width: fc.double({ noNaN: true }) })("matches a positive width", ({ width }) => {
    expect(isLineWithLength({ ...HELLO_LINE, width })).toBe(width > 0);
  });
});

import { describe, expect, it } from "vitest";

import { isAngleUnknown } from "./is-angle-unknown";

describe("isAngleUnknown (DERIV-handle-angle step 4)", () => {
  it("[F12.AC4] has no angle for a handle of length 0 typed as an angle, only then", () => {
    expect(isAngleUnknown("axis", { x: 0, y: 0 })).toBe(true);
    expect(isAngleUnknown("side", { x: 0, y: 0 })).toBe(true);
    expect(isAngleUnknown("side", { x: 1, y: 0 })).toBe(false);
    expect(isAngleUnknown("page", { x: 0, y: 0 })).toBe(false);
    expect(isAngleUnknown("point", { x: 0, y: 0 })).toBe(false);
  });
});

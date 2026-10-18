import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { isAutomaticLineHeight } from "./is-automatic-line-height";

describe("isAutomaticLineHeight (DERIV-text-box step 7)", () => {
  it.each([
    { isAutomatic: true, lineHeight: 0 },
    { isAutomatic: false, lineHeight: 1 },
    { isAutomatic: false, lineHeight: 24 },
  ])(
    "[F14.AC3] tells a line height of $lineHeight automatic: $isAutomatic",
    ({ isAutomatic, lineHeight }) => {
      expect(isAutomaticLineHeight({ ...HELLO_TEXT, lineHeight })).toBe(isAutomatic);
    },
  );
});

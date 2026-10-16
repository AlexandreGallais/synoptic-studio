import { describe, expect, it } from "vitest";

import { isNoBreakAfter } from "./is-no-break-after";

describe("isNoBreakAfter (REF-UAX14 LB12)", () => {
  it.each([
    { isExpected: true, unit: "\u{A0}" },
    { isExpected: false, unit: " " },
    { isExpected: false, unit: "H" },
    // A no-break space carrying a mark is a unit of its own, as a base character.
    { isExpected: false, unit: "\u{A0}\u{301}" },
  ])("[F14.AC1] gives $isExpected after '$unit'", ({ isExpected, unit }) => {
    expect(isNoBreakAfter(unit)).toBe(isExpected);
  });
});

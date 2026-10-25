import { describe, expect, it } from "vitest";

import { valueWithTyped } from "./value-with-typed";

describe("valueWithTyped (F38)", () => {
  it.each([
    // 100 + 50; 30 − 5; 460 set; 141 + 0.5 kept with its fraction; 7 set whatever the own.
    { expected: 150, own: 100, typed: { isRelative: true, value: 50 } },
    { expected: 25, own: 30, typed: { isRelative: true, value: -5 } },
    { expected: 460, own: 100, typed: { isRelative: false, value: 460 } },
    { expected: 141.5, own: 141, typed: { isRelative: true, value: 0.5 } },
    { expected: 7, own: NaN, typed: { isRelative: false, value: 7 } },
  ])("[F44.AC4] gives $expected from $own and $typed.value", ({ expected, own, typed }) => {
    expect(valueWithTyped(own, typed)).toBe(expected);
  });
});

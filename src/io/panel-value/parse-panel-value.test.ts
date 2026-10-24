import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { parsePanelValue } from "./parse-panel-value";

describe("parsePanelValue (F38, AUD-046)", () => {
  it.each([
    { expected: { isRelative: false, value: 460 }, typed: "460" },
    { expected: { isRelative: true, value: 50 }, typed: "+50" },
    { expected: { isRelative: true, value: -20 }, typed: "-20" },
    { expected: { isRelative: false, value: 0 }, typed: "0" },
    { expected: { isRelative: true, value: 0 }, typed: "+0" },
    { expected: { isRelative: false, value: 50 }, typed: " 50\t" },
    { expected: { isRelative: true, value: 7 }, typed: "+007" },
  ])("[F38.AC2] reads '$typed' as $expected.value", ({ expected, typed }) => {
    expect(parsePanelValue(typed)).toEqual(expected);
  });

  it.each([
    { expected: { isRelative: false, value: 50.5 }, typed: "50.5" },
    { expected: { isRelative: true, value: 1.5 }, typed: "+1.5" },
    { expected: { isRelative: true, value: -0.25 }, typed: "-.25" },
    { expected: { isRelative: false, value: 5 }, typed: "5." },
    { expected: { isRelative: false, value: 9_007_199_254_740_992 }, typed: "9007199254740992" },
  ])(
    "[F44.AC4] reads '$typed' as $expected.value, its fraction kept for the field",
    ({ expected, typed }) => {
      expect(parsePanelValue(typed)).toEqual(expected);
    },
  );

  it.each([
    "",
    " ",
    ".",
    "50,5",
    "1e3",
    "abc",
    "+",
    "-",
    "+-5",
    "--5",
    "5-",
    "5 0",
    "1.2.3",
    "50px",
    "0x10",
    "Infinity",
    "Mixed",
    // 400 digits: a number too large for a double, read as Infinity.
    "9".repeat(400),
  ])("[F44.AC4] refuses '%s', no number", (typed) => {
    expect(parsePanelValue(typed)).toBeUndefined();
  });

  test.prop([fc.maxSafeInteger().map((value) => Math.abs(value))])(
    "reads a natural number written as fixed, and with + or - as relative",
    (value) => {
      expect(parsePanelValue(String(value))).toEqual({ isRelative: false, value });
      expect(parsePanelValue(`+${value}`)).toEqual({ isRelative: true, value });
      expect(parsePanelValue(`-${value}`)).toEqual({ isRelative: true, value: -value });
    },
  );
});

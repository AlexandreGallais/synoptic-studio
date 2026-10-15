import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isColor } from "../../model";

import { parseHexColor } from "./parse-hex-color";

describe("parseHexColor (REF-CSS-COLOR-4 §5.2, DERIV-color-spaces step 1)", () => {
  it.each(["#FF8000", "ff8000", "  #ff8000\t", "#ff8000", "FF8000\n"])(
    "[F16.AC1] reads '%s' as #ff8000, no opacity given",
    (typed) => {
      expect(parseHexColor(typed)).toEqual({ color: "#ff8000", opacity: undefined });
    },
  );

  it.each([
    // Each digit doubled: f f, 8 8, 0 0.
    { expected: { color: "#ff8800", opacity: undefined }, typed: "#f80" },
    // Each digit doubled; `cc` = 204, ⌊(200 · 204 + 255) / 510⌋ = 80 %.
    { expected: { color: "#ff8800", opacity: 80 }, typed: "#F80C" },
    // CSS's own example: #0000ffcc is rgb(0 0 100% / 80%).
    { expected: { color: "#0000ff", opacity: 80 }, typed: "#0000ffcc" },
    // `ec` = 236, ⌊(200 · 236 + 255) / 510⌋ = ⌊93.05⌋ = 93 %.
    { expected: { color: "#000000", opacity: 93 }, typed: "#000000ec" },
    // `00` fully transparent, `ff` fully opaque.
    { expected: { color: "#123456", opacity: 0 }, typed: "12345600" },
    { expected: { color: "#123456", opacity: 100 }, typed: "#123456FF" },
  ])("[F16.AC1] reads '$typed' as $expected.color and its opacity", ({ expected, typed }) => {
    expect(parseHexColor(typed)).toEqual(expected);
  });

  it.each([
    "#ff800",
    "#ff80000",
    "#gg0000",
    "",
    " ".repeat(3),
    "#",
    "##ff8000",
    "#ff 8000",
    "#f8",
    "#ff8000ccc",
    "red",
    "rgb(255, 128, 0)",
  ])("[F16.AC1] refuses '%s'", (typed) => {
    expect(parseHexColor(typed)).toBeUndefined();
  });

  test.prop({ bytes: fc.array(fc.integer({ max: 255, min: 0 }), { maxLength: 3, minLength: 3 }) })(
    "[F16.AC1] reads a stored color again as itself, no opacity given",
    ({ bytes }) => {
      const color = `#${bytes.map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;

      expect(isColor(color)).toBe(true);
      expect(parseHexColor(color)).toEqual({ color, opacity: undefined });
    },
  );
});

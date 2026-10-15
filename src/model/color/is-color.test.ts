import { describe, expect, it } from "vitest";

import { isColor } from "./is-color";

describe("isColor (ADR-0056, REF-CSS-COLOR-4 §16.2.1)", () => {
  it.each(["#000000", "#ffffff", "#ff8000", "#1a2b3c"])("[F16.AC1] accepts %s", (color) => {
    expect(isColor(color)).toBe(true);
  });

  it.each([
    "#FF8000",
    "#1A2b3C",
    "",
    "#",
    "#fff",
    "000000",
    "#0000000",
    "#00000080",
    "#gggggg",
    " #000000",
    "red",
  ])("[F16.AC1] refuses '%s'", (color) => {
    expect(isColor(color)).toBe(false);
  });
});

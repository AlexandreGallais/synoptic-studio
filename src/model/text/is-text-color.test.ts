import { describe, expect, it } from "vitest";

import { isTextColor } from "./is-text-color";

describe("isTextColor (F05)", () => {
  it.each(["#000000", "#ffffff", "#1A2b3C"])("accepts %s", (color) => {
    expect(isTextColor(color)).toBe(true);
  });

  it.each(["", "#", "#fff", "000000", "#0000000", "#00000080", "#gggggg", " #000000", "red"])(
    "refuses '%s'",
    (color) => {
      expect(isTextColor(color)).toBe(false);
    },
  );
});

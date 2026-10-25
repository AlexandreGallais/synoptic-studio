import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isFontName } from "./is-font-name";

describe("isFontName (RULE-shapes-text)", () => {
  it.each(["Inter", "Open Sans", "Hawaii 5-0", "sans-serif"])("accepts '%s'", (name) => {
    expect(isFontName(name)).toBe(true);
  });

  it.each(["", "  ", " Inter", "Inter ", "Inter\nBold", "Inter\0", "A\u{7F}B"])(
    "refuses %j: empty, white space at an end, or a control character",
    (name) => {
      expect(isFontName(name)).toBe(false);
    },
  );

  test.prop({ name: fc.string({ minLength: 1 }) })(
    "accepts a name only when trimming leaves it as it is",
    ({ name }) => {
      expect(!isFontName(name) || name.trim() === name).toBe(true);
    },
  );
});

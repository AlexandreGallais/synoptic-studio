import { describe, expect, it } from "vitest";

import { screenToUser } from "./screen-to-user";

describe("screenToUser (RULE-interaction-the-outline-screen-pixels)", () => {
  it.each([
    // At scale 1 a pixel is a user unit; zoomed in twice, 4 pixels are 2 units; zoomed out to ½,
    // 2 pixels are 4 units.
    { expected: 4, pixels: 4, scale: 1 },
    { expected: 2, pixels: 4, scale: 2 },
    { expected: 4, pixels: 2, scale: 0.5 },
    { expected: 0, pixels: 0, scale: 3 },
  ])(
    "gives $pixels pixels at scale $scale as $expected user units",
    ({ expected, pixels, scale }) => {
      expect(screenToUser(pixels, scale)).toBe(expected);
    },
  );
});

import { describe, expect, it } from "vitest";

import { isBrowserKey } from "./is-browser-key";

describe("isBrowserKey (AUD-045)", () => {
  it.each([
    { altKey: false, ctrlKey: true, metaKey: false },
    { altKey: false, ctrlKey: false, metaKey: true },
    { altKey: true, ctrlKey: false, metaKey: false },
    { altKey: true, ctrlKey: true, metaKey: true },
  ])(
    "[F43.AC3] leaves to the browser a key with Ctrl $ctrlKey, Cmd $metaKey, Alt $altKey",
    (modifiers) => {
      expect(isBrowserKey(modifiers)).toBe(true);
    },
  );

  it("[F43.AC3] keeps for the page a key without Ctrl, Cmd or Alt, Shift whatever it is", () => {
    expect(isBrowserKey({ altKey: false, ctrlKey: false, metaKey: false })).toBe(false);
  });
});

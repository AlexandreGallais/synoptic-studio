import { describe, expect, it } from "vitest";

import { NO_HANDLES } from "../../geometry";

import { handlesAfterTyping } from "./handles-after-typing";

describe("handlesAfterTyping", () => {
  it("[F12.AC2] sets the handle typed and mirrors the other in mode angle and length", () => {
    // After (30, −15) typed: before becomes (−30, 15).
    expect(handlesAfterTyping(NO_HANDLES, "after", { x: 30, y: -15 })).toEqual({
      ...NO_HANDLES,
      after: { x: 30, y: -15 },
      before: { x: -30, y: 15 },
    });
  });

  it("[F12.AC2] keeps the other handle's length in mode angle, from the handle before too", () => {
    // Before (30, −40) typed, |after| = 10: after becomes (−6, 8).
    const handles = { ...NO_HANDLES, after: { x: 10, y: 0 }, mode: "angle" } as const;

    expect(handlesAfterTyping(handles, "before", { x: 30, y: -40 })).toEqual({
      ...handles,
      after: { x: -6, y: 8 },
      before: { x: 30, y: -40 },
    });
  });

  it("[F12.AC2] leaves the other handle in mode none, and the other handle when the one typed has no length in mode angle", () => {
    const none = { ...NO_HANDLES, before: { x: -5, y: 7 }, mode: "none" } as const;
    const angle = { ...none, mode: "angle" } as const;

    expect(handlesAfterTyping(none, "after", { x: 30, y: -15 }).before).toEqual({ x: -5, y: 7 });
    expect(handlesAfterTyping(angle, "after", { x: 0, y: 0 }).before).toEqual({ x: -5, y: 7 });
  });

  it("keeps the switch and the mode", () => {
    const on = { ...NO_HANDLES, isOn: true, mode: "angle" } as const;
    const typed = handlesAfterTyping(on, "after", { x: 3, y: 4 });

    expect([typed.isOn, typed.mode]).toEqual([true, "angle"]);
  });

  it("mirrors from the handle before too, in modes angle and length and none", () => {
    // Before (30, −15) typed: after becomes (−30, 15) in angle and length, stays (−5, 7) in none.
    const kept = { ...NO_HANDLES, after: { x: -5, y: 7 } };

    expect(handlesAfterTyping(kept, "before", { x: 30, y: -15 }).after).toEqual({ x: -30, y: 15 });

    expect(
      handlesAfterTyping({ ...kept, mode: "none" }, "before", { x: 30, y: -15 }).after,
    ).toEqual({
      x: -5,
      y: 7,
    });
  });
});

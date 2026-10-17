import { describe, expect, it } from "vitest";

import { NO_HANDLES } from "../../geometry";

import { areValidHandles } from "./are-valid-handles";

import type { Handles } from "../../geometry";

describe("areValidHandles (ADR-0046)", () => {
  it("accepts the handles of a new corner, and handles on with exact vectors", () => {
    expect(areValidHandles(NO_HANDLES)).toBe(true);

    expect(
      areValidHandles({ ...NO_HANDLES, after: { x: 25.98076, y: -15 }, isOn: true, mode: "angle" }),
    ).toBe(true);
  });

  it("refuses an unknown mode, a vector not finite, a switch not boolean", () => {
    // Read from outside (the page's stored JSON): the type does not hold there.
    // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- data read from outside, as the playground does
    const unknown = JSON.parse('{"isOn":false,"mode":"smooth"}') as Handles;
    // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- data read from outside, as the playground does
    const notBoolean = JSON.parse('{"isOn":1}') as Handles;

    expect(areValidHandles({ ...NO_HANDLES, mode: unknown.mode })).toBe(false);
    expect(areValidHandles({ ...NO_HANDLES, before: { x: NaN, y: 0 } })).toBe(false);
    expect(areValidHandles({ ...NO_HANDLES, after: { x: 0, y: Infinity } })).toBe(false);
    expect(areValidHandles({ ...NO_HANDLES, isOn: notBoolean.isOn })).toBe(false);
  });
});

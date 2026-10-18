import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { pickedColorToHex } from "./picked-color-to-hex";

/**
 * The byte of a component `x / d` rounded to the closest, a tie up, in exact integers: the
 * reference the library is checked against (`DERIV-color-spaces`, step 11).
 *
 * @param x - the numerator, in BigInt
 * @param d - the denominator, in BigInt
 * @returns ⌊(2 · 255 · x + d) / 2d⌋
 */
function exactByte(x: bigint, d: bigint): number {
  return Number((510n * x + d) / (2n * d));
}

/**
 * The smaller of two exact integers (`Math.min` takes no BigInt).
 *
 * @param a - one integer
 * @param b - the other
 * @returns the smaller of the two
 */
function smallerBig(a: bigint, b: bigint): bigint {
  return [a, b].toSorted((x, y) => Number(x - y))[0] ?? a;
}

/**
 * An exact integer kept within a range.
 *
 * @param x - the integer
 * @param low - the lower bound
 * @param high - the upper bound
 * @returns the integer in the range
 */
function clampBig(x: bigint, low: bigint, high: bigint): bigint {
  return -smallerBig(-smallerBig(x, high), -low);
}

/**
 * HSB's `f(n)` in exact rationals, the values in tenths: `v / 1000 · (1 − s / 1000 · c)`, with
 * `c = clamp(min(k, 4 − k), 0, 1)` and `k = (n + h / 600) mod 6`, written over 6 · 10⁸.
 *
 * @param tenths - hue, saturation and brightness in tenths
 * @param n - 5 for red, 3 for green, 1 for blue
 * @returns the component's byte, rounded a tie up
 */
function exactHsbByte(tenths: readonly [bigint, bigint, bigint], n: number): number {
  const [h, s, v] = tenths;
  const k = (600n * BigInt(n) + h) % 3600n;
  const c = clampBig(smallerBig(k, 2400n - k), 0n, 600n);

  return exactByte(v * (600_000n - s * c), 600_000_000n);
}

/**
 * HSL's `f(n)` in exact rationals, the values in tenths:
 * `l / 1000 − s / 1000 · min(l, 1000 − l) / 1000 · q / 300`, with
 * `q = clamp(min(K − 900, 2700 − K), −300, 300)`, written over 3 · 10⁸.
 *
 * @param tenths - hue, saturation and lightness in tenths
 * @param n - 0 for red, 8 for green, 4 for blue
 * @returns the component's byte, rounded a tie up
 */
function exactHslByte(tenths: readonly [bigint, bigint, bigint], n: number): number {
  const [h, s, light] = tenths;
  const k = (300n * BigInt(n) + h) % 3600n;
  const q = clampBig(smallerBig(k - 900n, 2700n - k), -300n, 300n);
  const halfChroma = smallerBig(light, 1000n - light);

  return exactByte(300_000n * light - s * halfChroma * q, 300_000_000n);
}

describe("pickedColorToHex (DERIV-color-spaces steps 6, 8 and 11)", () => {
  it.each([
    // HSB 210°, 100 %, 50 %: red 0; green 0.25, 63.75 gives 64 (40); blue 0.5, 127.5 gives 128.
    { expected: "#004080", model: "hsb", picked: { hue: 210, level: 50, saturation: 100 } },
    // HSB 210°, 67 %, 60 %: red 0.6 − 0.6 · 0.67 = 0.198, 50.49 gives 50 (32): not #336699.
    { expected: "#326699", model: "hsb", picked: { hue: 210, level: 60, saturation: 67 } },
    // HSB 210°, 67 %, 61 %: red 0.2013, 51.33 → 51 (33); green 0.40565, 103.44 → 103 (67).
    { expected: "#33679c", model: "hsb", picked: { hue: 210, level: 61, saturation: 67 } },
    // HSL 30°, 100 %, 50 %: green 0.5, 127.5 gives 128 (80).
    { expected: "#ff8000", model: "hsl", picked: { hue: 30, level: 50, saturation: 100 } },
    { expected: "#ff8000", model: "hsb", picked: { hue: 30, level: 100, saturation: 100 } },
    // A grey: HSB 200°, 0 %, 50 %, 127.5 gives 128 (80).
    { expected: "#808080", model: "hsb", picked: { hue: 200, level: 50, saturation: 0 } },
    // HSB 200°, 50 %, 100 %: red 0.5, 127.5 gives 128 (80); green 5/6, 212.5 gives 213 (d5).
    { expected: "#80d5ff", model: "hsb", picked: { hue: 200, level: 100, saturation: 50 } },
    // HSB 0°, 75 %, 40 %: red 0.4, 102 (66); green and blue 0.1, 25.5 gives 26 (1a).
    { expected: "#661a1a", model: "hsb", picked: { hue: 0, level: 40, saturation: 75 } },
    // HSL 0°, 75 %, 40 %: red 0.7, 178.5 gives 179 (b3); green and blue 0.1, 26 (1a).
    { expected: "#b31a1a", model: "hsl", picked: { hue: 0, level: 40, saturation: 75 } },
    // HSL 0°, 75 %, 60 %: red 0.9, 229.5 gives 230 (e6); green and blue 0.3, 76.5 gives 77 (4d).
    { expected: "#e64d4d", model: "hsl", picked: { hue: 0, level: 60, saturation: 75 } },
  ] as const)("[F16.AC2] gives $expected for $picked in $model", ({ expected, model, picked }) => {
    expect(pickedColorToHex(picked, model)).toBe(expected);
  });

  test.prop({
    h: fc.integer({ max: 359, min: 0 }),
    s: fc.integer({ max: 100, min: 0 }),
    v: fc.integer({ max: 100, min: 0 }),
  })("gives the bytes of the exact rational arithmetic in HSB", ({ h, s, v }) => {
    // An integer is ten tenths.
    const tenths = [10n * BigInt(h), 10n * BigInt(s), 10n * BigInt(v)] as const;
    const bytes = [5, 3, 1].map((n) => exactHsbByte(tenths, n).toString(16).padStart(2, "0"));

    expect(pickedColorToHex({ hue: h, level: v, saturation: s }, "hsb")).toBe(`#${bytes.join("")}`);
  });

  test.prop({
    h: fc.integer({ max: 359, min: 0 }),
    l: fc.integer({ max: 100, min: 0 }),
    s: fc.integer({ max: 100, min: 0 }),
  })("gives the bytes of the exact rational arithmetic in HSL", ({ h, l, s }) => {
    const tenths = [10n * BigInt(h), 10n * BigInt(s), 10n * BigInt(l)] as const;
    const bytes = [0, 8, 4].map((n) => exactHslByte(tenths, n).toString(16).padStart(2, "0"));

    expect(pickedColorToHex({ hue: h, level: l, saturation: s }, "hsl")).toBe(`#${bytes.join("")}`);
  });
});

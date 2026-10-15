// Colors shared by the tests of `src/` (EN-038): a color's components from its three bytes, and
// any color for properties. Tests only: never part of the library.

import { fc } from "@fast-check/vitest";

import { DEGREES_PER_TURN, fractionToByte, integerRange } from "../src";

import type { IntegerRatio, Rgb } from "../src";

/** The largest byte, `ff`. */
const BYTE_MAX = 255;

/**
 * A color's components from its bytes, each divided by 255 (`DERIV-color-spaces`, notation).
 *
 * @param red - the red byte, from 0 to 255
 * @param green - the green byte, from 0 to 255
 * @param blue - the blue byte, from 0 to 255
 * @returns the components, from 0 to 1
 */
export function rgbOfBytes(red: number, green: number, blue: number): Rgb {
  return { blue: blue / BYTE_MAX, green: green / BYTE_MAX, red: red / BYTE_MAX };
}

/** Any byte, from 0 to 255. */
const ANY_BYTE = fc.integer({ max: BYTE_MAX, min: 0 });

/** Any of the 16 777 216 colors, as its bytes. */
export const ANY_BYTE_RGB = fc.record({ blue: ANY_BYTE, green: ANY_BYTE, red: ANY_BYTE });

/** Any of the 16 777 216 colors, as its components. */
export const ANY_RGB = ANY_BYTE_RGB.map(({ blue, green, red }) => rgbOfBytes(red, green, blue));

/**
 * Any fraction of integers from 0 to a whole, its denominator up to a bound.
 *
 * @param whole - the largest value, 1 for a fraction, 360 for a hue in degrees
 * @param largestDenominator - the bound of the denominator
 * @returns fractions `n / d`, `d` from 1 to the bound, `n` from 0 to `whole · d`
 */
function anyRatio(whole: number, largestDenominator: number): fc.Arbitrary<IntegerRatio> {
  return fc
    .integer({ max: largestDenominator, min: 1 })
    .chain((denominator) =>
      fc
        .integer({ max: whole * denominator, min: 0 })
        .map((numerator) => ({ denominator, numerator })),
    );
}

/**
 * Any exact color whose denominators are those a color read from its bytes, or a value typed, may
 * have: the hue over the chroma (up to 255), the saturation over up to 255, the level over up to
 * 510 (`DERIV-color-spaces`, step 12).
 */
export const ANY_EXACT_COLOR = fc.record({
  hue: anyRatio(DEGREES_PER_TURN, BYTE_MAX),
  level: anyRatio(1, 2 * BYTE_MAX),
  saturation: anyRatio(1, BYTE_MAX),
});

/** The values of a byte: 256. */
const BYTE_COUNT = 256;

/** The colors there are: 256³ = 16 777 216. */
const COLOR_COUNT = 16_777_216;

/**
 * One color in this many is drawn into the sample, 31 being prime to 256 so that every byte of
 * every component is drawn.
 */
const SAMPLE_STRIDE = 31;

/**
 * A color's index, `65 536 · R + 256 · G + B`.
 *
 * @param red - its red byte
 * @param green - its green byte
 * @param blue - its blue byte
 * @returns the index, from 0 to 16 777 215
 */
function colorIndex(red: number, green: number, blue: number): number {
  return (red * BYTE_COUNT + green) * BYTE_COUNT + blue;
}

/**
 * The colors of the round trip's sample, by their index: one color in 31, and every color with
 * two equal components, where the hue changes sector (greys among them), about 731 000 of the
 * 16 777 216.
 *
 * @returns the indices, each once
 */
function sampledColorIndices(): readonly number[] {
  const strided = integerRange(0, Math.ceil(COLOR_COUNT / SAMPLE_STRIDE) - 1).map(
    (k) => SAMPLE_STRIDE * k,
  );
  const bytes = integerRange(0, BYTE_COUNT - 1);
  const edges = bytes.flatMap((a) =>
    bytes.flatMap((b) => [colorIndex(a, a, b), colorIndex(b, a, a), colorIndex(a, b, a)]),
  );

  return [...new Set([...strided, ...edges])];
}

/** The sampled colors' indices, computed once for every round trip. */
const SAMPLED_COLOR_INDICES = sampledColorIndices();

/**
 * Whether a round trip gives a color's bytes back: its components converted and back, each
 * brought to its byte (`fractionToByte`).
 *
 * @param roundTrip - the components of a color to the components it comes back as
 * @param index - the color's index
 * @returns `true` when every byte comes back
 */
function isKept(roundTrip: (rgb: Rgb) => Rgb, index: number): boolean {
  const blue = index % BYTE_COUNT;
  const green = Math.floor(index / BYTE_COUNT) % BYTE_COUNT;
  const red = Math.floor(index / BYTE_COUNT / BYTE_COUNT);
  const back = roundTrip(rgbOfBytes(red, green, blue));

  return (
    [back.red, back.green, back.blue].map((component) => fractionToByte(component)).join(",") ===
    [red, green, blue].join(",")
  );
}

/**
 * How many of the sampled colors (`sampledColorIndices`) a round trip changes. Every color was run
 * once, at EN-038, 13 to 17 s a round trip: too slow for each run (`DERIV-color-spaces`,
 * cross-check).
 *
 * @param roundTrip - the components of a color to the components it comes back as
 * @returns the number of sampled colors whose bytes changed
 */
export function changedColorCount(roundTrip: (rgb: Rgb) => Rgb): number {
  return SAMPLED_COLOR_INDICES.filter((index) => !isKept(roundTrip, index)).length;
}

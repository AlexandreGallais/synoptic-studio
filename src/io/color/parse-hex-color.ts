import { alphaToPercent } from "./alpha-to-percent";
import { hexPairToByte } from "./hex-pair-to-byte";

/** A color typed in hexadecimal, read into its stored form. */
export type TypedColor = {
  /** The color, `#` and six lower-case hexadecimal digits. */
  readonly color: string;
  /** The opacity its alpha gives, an integer percent; `undefined` when it had no alpha. */
  readonly opacity: number | undefined;
};

/** The digits of a hex color, the `#` taken away: 3, 4, 6 or 8 of them. */
const HEX_DIGITS = /^(?:[\da-f]{3,4}|[\da-f]{6}|[\da-f]{8})$/u;

/** A `#` at the start of a hex color, which may be left out. */
const LEADING_HASH = /^#/u;

/** Each digit of a short form, written twice by `$&$&`. */
const EACH_DIGIT = /[\da-f]/gu;

/** The most digits of a short form, `RGBA`, whose digits are doubled. */
const SHORT_FORM_LENGTH = 4;

/** The digits of red, green and blue in the full form, the alpha's after them. */
const RGB_LENGTH = 6;

/**
 * Reads the digits of a hex color in their full form, 6 or 8 lower-case digits, each digit of a
 * 3- or 4-digit form doubled (REF-CSS-COLOR-4, §5.2: `#123` is `#112233`).
 *
 * @kind format
 * @param typed - the text typed or pasted
 * @returns the full digits; empty when the text is no hex color
 * @see REF-CSS-COLOR-4
 */
function parseHexDigits(typed: string): string {
  const digits = typed.trim().toLowerCase().replace(LEADING_HASH, "");

  if (!HEX_DIGITS.test(digits)) {
    return "";
  }

  return digits.length > SHORT_FORM_LENGTH ? digits : digits.replaceAll(EACH_DIGIT, "$&$&");
}

/**
 * Reads a color typed or pasted in any hexadecimal form CSS knows into its stored form, `#` and
 * six lower-case digits, with the opacity its alpha gives, if any: the `#` may be left out, the
 * letters in any case, white space around ignored; 6 digits are red, green and blue, 8 add the
 * alpha, 3 and 4 the same with each digit doubled (REF-CSS-COLOR-4, §5.2 and §16.2.1;
 * `DERIV-color-spaces`, step 1). `#F80C` is `#ff8800` at 80 %. CSS names and `rgb()` are refused.
 *
 * @kind format
 * @param typed - the text typed or pasted
 * @returns the color and its opacity; `undefined` when the text is no hex color
 * @see REF-CSS-COLOR-4
 */
export function parseHexColor(typed: string): TypedColor | undefined {
  const digits = parseHexDigits(typed);

  if (digits === "") {
    return undefined;
  }

  const alpha = digits.slice(RGB_LENGTH);

  return {
    color: `#${digits.slice(0, RGB_LENGTH)}`,
    opacity: alpha === "" ? undefined : alphaToPercent(hexPairToByte(alpha)),
  };
}

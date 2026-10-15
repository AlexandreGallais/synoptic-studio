import { formatSvgNumber } from "../path-data";

/**
 * A length of the HTML layer in CSS pixels: inside a `foreignObject`, which is "subject to SVG
 * transformations", a CSS pixel is a user unit (`REF-SVG2-EMBEDDED`, §12.4), so the length is
 * written as the SVG writes its numbers (`formatSvgNumber`): 18.3984375 gives `18.39844px`.
 *
 * @kind format
 * @param length - a length in user units
 * @returns the length, followed by `px`
 * @see REF-SVG2-EMBEDDED
 */
export function formatCssPixels(length: number): string {
  return `${formatSvgNumber(length)}px`;
}

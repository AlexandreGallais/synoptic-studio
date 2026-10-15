import { bytesToHex, percentToAlpha } from "../color";

/**
 * A color and its opacity as one CSS color: `#`, the six digits of the color, then the two of its
 * alpha byte (`percentToAlpha`), the last pair of an eight-digit hexadecimal color
 * (`REF-CSS-COLOR-4`, §5.2): `#22c55e` at 40 % gives `#22c55e66`, at 100 % `#22c55eff`. The
 * opacity stays with the color, so that a text's and its background's do not multiply as an
 * element's `opacity` would.
 *
 * @kind format
 * @param color - the stored color, `#` and six lower-case digits
 * @param opacity - its opacity, an integer from 0 to 100 %
 * @returns the color with its alpha
 * @see REF-CSS-COLOR-4
 */
export function formatCssColor(color: string, opacity: number): string {
  return `${color}${bytesToHex([percentToAlpha(opacity)]).slice(1)}`;
}

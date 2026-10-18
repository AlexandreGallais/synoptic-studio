import { BYTE_MAXIMUM } from "../../math";

/**
 * A color component's byte as a fraction from 0 to 1, the byte divided by 255, the inverse of
 * `fractionToByte`: `33` (51) is 0.2, `ff` is 1 (REF-CSS-COLOR-4, §5.2: `00` to `ff` for 0 % to
 * 100 %; `DERIV-color-spaces`, notation: `r = R / 255`).
 *
 * Formula: x = B / 255.
 *
 * @kind format
 * @param byte - the component's byte, an integer from 0 to 255
 * @returns the component, from 0 to 1
 * @see REF-CSS-COLOR-4
 */
export function byteToFraction(byte: number): number {
  return byte / BYTE_MAXIMUM;
}

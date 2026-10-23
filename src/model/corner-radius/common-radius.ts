import { commonValue } from "../../math";

/**
 * Radius shared by every vertex of a shape, shown in its radius field; none when the radii
 * differ, the field then showing "Mixed" (Figma's `figma.mixed`, `REF-FIGMA-CR`), or when the
 * shape has no vertex: the radii compared exactly (`commonValue` within 0).
 *
 * @kind domain
 * @param radii - requested radius of each vertex, integers ≥ 0 (`areValidRadii`): `NaN` equals
 * nothing, and `-0` equals `0`
 * @returns the common radius, `undefined` when there is none (an absent value, an option type in
 * other languages, ADR-0025)
 * @see REF-FIGMA-CR
 */
export function commonRadius(radii: readonly number[]): number | undefined {
  return commonValue(radii, 0);
}

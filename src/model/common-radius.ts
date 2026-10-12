/**
 * Radius shared by every vertex of a shape, shown in its radius field; none when the radii
 * differ, the field then showing "Mixed" (Figma's `figma.mixed`, `REF-FIGMA-CR`), or when the
 * shape has no vertex.
 *
 * @kind domain
 * @param radii - requested radius of each vertex, integers ≥ 0 (`areValidRadii`): compared with
 * `===`, under which `NaN` equals nothing, and `-0` equals `0`
 * @returns the common radius, `undefined` when there is none (an absent value, an option type in
 * other languages, ADR-0025)
 * @see REF-FIGMA-CR
 */
export function commonRadius(radii: readonly number[]): number | undefined {
  const [first] = radii;

  return radii.every((radius) => radius === first) ? first : undefined;
}

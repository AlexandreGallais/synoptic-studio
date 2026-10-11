/**
 * Radius shared by every vertex of a shape, shown in its radius field; none when the radii
 * differ, the field then showing "Mixed" (Figma's `figma.mixed`, `REF-FIGMA-CR`), or when the
 * shape has no vertex.
 *
 * @kind domain
 * @param radii - requested radius of each vertex
 * @returns the common radius, `undefined` when there is none
 * @see REF-FIGMA-CR
 */
export function commonRadius(radii: readonly number[]): number | undefined {
  const [first] = radii;

  return radii.every((radius) => radius === first) ? first : undefined;
}

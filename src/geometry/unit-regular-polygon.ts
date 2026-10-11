import type { Point } from "../math";

/**
 * Vertices of a regular polygon on the unit circle centered at the origin, with a horizontal
 * bottom edge (a flat base), in the SVG frame (y pointing down).
 *
 * The vertices run clockwise on screen from the right end of the flat base (Q18, F08): the
 * `j`-th is at the angle `βⱼ = −π/2 + (1 − 2j) π / n` of the mathematical frame, so that the
 * base is always vertices 1 and 2 (indexes 0 and 1),
 * written `(cos βⱼ, −sin βⱼ)` once the y axis is flipped. The start is chosen by its index,
 * never by comparing floating coordinates. No coordinate is `−0`; one that is zero in exact
 * arithmetic may be off by about 1e-16 (cosine and sine of a double), which is accepted.
 *
 * @kind geometry
 * @param corners - number of corners `n`, an integer >= 3
 * @returns the `n` vertices in drawing order
 * @see DERIV-regular-polygon-fit
 */
export function unitRegularPolygon(corners: number): readonly Point[] {
  return Array.from({ length: corners }, (empty, index) => {
    const angle = -Math.PI / 2 + ((1 - 2 * index) * Math.PI) / corners;

    return { x: Math.cos(angle), y: 0 - Math.sin(angle) };
  });
}

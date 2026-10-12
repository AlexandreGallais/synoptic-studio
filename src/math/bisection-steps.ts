/**
 * Halvings of a bisection: a bracket of 6 shrinks to `6 · 2⁻⁶⁰ ≈ 5e-18` (`REF-MATHWORLD-BISECTION`),
 * far below `EPSILON`; in doubles the bracket stops shrinking at one unit in the last place.
 */
export const BISECTION_STEPS = 60;

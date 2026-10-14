/**
 * Halvings of the searches of a fillet beside a curve: a bracket of 1 shrinks to 2⁻⁴⁰ ≈ 1e-12
 * (`REF-MATHWORLD-BISECTION`), far below the 5 decimals drawn (`SVG_DECIMALS`), at two thirds of
 * the cost of `BISECTION_STEPS`: these searches nest.
 */
export const FILLET_STEPS = 40;

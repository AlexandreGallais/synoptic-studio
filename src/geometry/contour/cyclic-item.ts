/**
 * Item of a cyclic list at any integer index: indices are taken modulo the list length (Q11).
 *
 * Index −1 is the last item, index `n` the first one. The index is reduced with a non-negative
 * modulo, `((i mod n) + n) mod n`, not with the sign-keeping `%` of JavaScript alone (ADR-0025).
 *
 * @kind geometry
 * @template T - type of the items: a vertex, or a value per vertex
 * @param items - vertices of a contour, or one value per vertex, in drawing order
 * @param index - position, wrapped around the list
 * @param fallback - value returned when the list is empty
 * @returns the item at `index mod n`, or `fallback` for an empty list
 * @see DERIV-turning-angle
 */
export function cyclicItem<T>(items: readonly T[], index: number, fallback: T): T {
  return items.at(((index % items.length) + items.length) % items.length) ?? fallback;
}

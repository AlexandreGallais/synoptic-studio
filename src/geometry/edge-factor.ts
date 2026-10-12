/**
 * Factor by which the setbacks at both ends of an edge are reduced so that they fit the edge.
 *
 * Formula: f = 1 if S ≤ L, otherwise f = L / S. The comparison comes first, so a zero-length edge
 * without demand gives 1, never a division by zero.
 *
 * @kind geometry
 * @param length - length `L` of the edge, >= 0
 * @param demand - sum `S` of the setbacks requested at its two ends, >= 0
 * @returns factor in [0, 1]: 0 when a zero-length edge carries a demand
 * @see DERIV-local-radius-clamp
 */
export function edgeFactor(length: number, demand: number): number {
  return demand <= length ? 1 : length / demand;
}

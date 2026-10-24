/**
 * Whether a piece going from one ordinate to another meets the horizontal line at a level, by the
 * edge crossing rules: a piece going up the axis includes its start and excludes its end, one
 * going down excludes its start and includes its end, a flat one never meets it, so that a vertex
 * on the line is counted once or not at all (`DERIV-painted-area` step 2).
 *
 * Formula: min(a, b) ≤ level < max(a, b).
 *
 * @kind geometry
 * @param from - ordinate of the piece's start
 * @param to - ordinate of the piece's end
 * @param level - ordinate of the line
 * @returns true when the piece meets the line under those rules
 * @see REF-SUNDAY-INCLUSION
 */
export function isLevelCrossed(from: number, to: number, level: number): boolean {
  return Math.min(from, to) <= level && level < Math.max(from, to);
}

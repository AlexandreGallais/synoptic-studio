/** The no-break space U+00A0, of the class GL of UAX #14. */
const NO_BREAK_SPACE = "\u{A0}";

/**
 * Whether a line may never be cut right after a unit: after a no-break space U+00A0, "do not
 * break after" a character of class GL (UAX #14, LB12), so that it stays with the unit after it,
 * whether a word too long is broken or a line is cut for its ellipsis (`DERIV-text-box`, steps 4
 * and 6).
 *
 * @kind domain
 * @param unit - a typographic character unit of a line (`characterUnits`)
 * @returns `true` for a no-break space
 * @see REF-UAX14
 */
export function isNoBreakAfter(unit: string): boolean {
  return unit === NO_BREAK_SPACE;
}

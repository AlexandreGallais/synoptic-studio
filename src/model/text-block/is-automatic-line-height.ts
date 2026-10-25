import type { Text } from "../text";

/** The line height that asks for the font's own. */
const AUTOMATIC = 0;

/**
 * Whether a text's line height is automatic, the font's own, written 0 (provisional,
 * VAL-015), rather than an integer chosen from 1 (`DERIV-text-box`, step 7).
 *
 * @kind domain
 * @param text - its line height
 * @returns true when its line height is 0, the font's own
 * @see DERIV-text-box
 */
export function isAutomaticLineHeight(text: Text): boolean {
  return text.lineHeight === AUTOMATIC;
}

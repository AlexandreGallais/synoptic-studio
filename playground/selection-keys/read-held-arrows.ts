import { HELD_ARROWS } from "./held-arrows";

/**
 * Reads the arrows held outside the panel's fields (`HELD_ARROWS`, US-081).
 *
 * @kind procedure
 * @param document - page document
 * @returns their keys, as `KeyboardEvent.key` names them, in the order pressed; none held
 * @see RULE-interaction-2-selector
 */
export function readHeldArrows(document: Document): readonly string[] {
  return HELD_ARROWS.get(document) ?? [];
}

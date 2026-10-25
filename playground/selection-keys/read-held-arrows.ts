import { HELD_ARROWS } from "./held-arrows";

/**
 * Reads the arrows held outside the panel's fields (`HELD_ARROWS`).
 *
 * @kind procedure
 * @param document - page document
 * @returns their keys, as `KeyboardEvent.key` names them, in the order pressed; none held
 */
export function readHeldArrows(document: Document): readonly string[] {
  return HELD_ARROWS.get(document) ?? [];
}

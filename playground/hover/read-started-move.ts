import { SELECTION_DRAGS } from "./selection-drags";

import type { ShapesMove } from "./shapes-move";

/**
 * Reads the move of shapes under way (F39): a press on a shape whose pointer has gone further than
 * 3 pixels from it, from then to the release, the shapes following the pointer.
 *
 * @kind procedure
 * @param document - page document
 * @returns the move; undefined before it starts, for another press, or for none
 * @see RULE-interaction-2-selector
 */
export function readStartedMove(document: Document): ShapesMove | undefined {
  const drag = SELECTION_DRAGS.get(document);

  return drag?.isMoved === true ? drag.move : undefined;
}

import { mirroredHandle } from "../../geometry";

import type { Handles } from "../../geometry";
import type { Vector } from "../../math";

/**
 * A point's handles once one of them is typed: that handle set, the other following the mode
 * (`mirroredHandle`); the switch unchanged, the mode too but `auto`, which becomes `angle`: a handle
 * typed leaves the tension, its handles kept on one line (the agent's choice, US-026).
 *
 * @kind domain
 * @param handles - handles before the change, as drawn: the shape at scale 1 (`freeShapeAroundCorners`), so that mode angle keeps the length seen
 * @param end - the handle typed: towards the previous point (`before`) or the next (`after`)
 * @param typed - its new vector from the point, as drawn
 * @returns the handles after the change
 * @see docs/domain/shapes.md
 */
export function handlesAfterTyping(
  handles: Handles,
  end: "after" | "before",
  typed: Vector,
): Handles {
  const mode = handles.mode === "auto" ? "angle" : handles.mode;

  return end === "after"
    ? {
        ...handles,
        after: typed,
        before: mirroredHandle(handles.mode, typed, handles.before),
        mode,
      }
    : {
        ...handles,
        after: mirroredHandle(handles.mode, typed, handles.after),
        before: typed,
        mode,
      };
}

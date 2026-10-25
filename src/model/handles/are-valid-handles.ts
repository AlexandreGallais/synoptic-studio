import { HANDLE_MODES } from "../../geometry";

import type { Handles } from "../../geometry";

/**
 * Whether a point's handles can enter the model: a switch, a known mode and two finite vectors
 * (ADR-0046); stored exactly, as the points of a free shape (ADR-0034).
 *
 * @kind domain
 * @param handles - handles to check, read from outside
 * @returns `true` when every value is allowed
 * @see RULE-shapes
 */
export function areValidHandles(handles: Handles): boolean {
  return (
    typeof handles.isOn === "boolean" &&
    HANDLE_MODES.includes(handles.mode) &&
    [handles.before, handles.after].every(
      (vector) => Number.isFinite(vector.x) && Number.isFinite(vector.y),
    )
  );
}

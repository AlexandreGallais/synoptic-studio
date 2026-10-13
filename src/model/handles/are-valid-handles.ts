import { HANDLE_MODES } from "../../geometry";

import type { Handles } from "../../geometry";

/** A hundred: the largest tension, a whole percentage. */
const PERCENT = 100;

/**
 * Whether a value is a tension: a whole percentage from 0 to 100 (`DERIV-handle-tension`).
 *
 * @kind domain
 * @param value - the value to check
 * @returns `true` for an integer from 0 to 100
 * @see DERIV-handle-tension
 */
function isTension(value: number): boolean {
  return Number.isSafeInteger(value) && value >= 0 && value <= PERCENT;
}

/**
 * Whether a point's handles can enter the model: a switch, a known mode, a tension from 0 to 100
 * and two finite vectors
 * (ADR-0046); stored exactly, as the points of a free shape (ADR-0034).
 *
 * @kind domain
 * @param handles - handles to check, read from outside
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md
 */
export function areValidHandles(handles: Handles): boolean {
  return (
    typeof handles.isOn === "boolean" &&
    HANDLE_MODES.includes(handles.mode) &&
    isTension(handles.tension) &&
    [handles.before, handles.after].every(
      (vector) => Number.isFinite(vector.x) && Number.isFinite(vector.y),
    )
  );
}

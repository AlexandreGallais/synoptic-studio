import { fieldArrowStep, isBrowserKey } from "../../src";

import type { TypedPanelValue } from "../../src";

/**
 * Reads the step an arrow pressed in a field acting as a number field gives its value (US-081,
 * US-085): Up adds 1, Down takes 1 away, 10 with Shift (`fieldArrowStep`), as `+1` or `-10`
 * typed; none for another key, nor with Ctrl, Cmd or Alt held (`isBrowserKey`), which the field
 * keeps.
 *
 * @kind procedure
 * @param event - the key pressed in the field
 * @returns the step, relative; undefined for a key that steps nothing
 * @see docs/domain/interaction.md#_2-selector
 */
export function readArrowStep(event: KeyboardEvent): TypedPanelValue | undefined {
  const step = fieldArrowStep(event.key, event.shiftKey);

  return step === undefined || isBrowserKey(event) ? undefined : { isRelative: true, value: step };
}

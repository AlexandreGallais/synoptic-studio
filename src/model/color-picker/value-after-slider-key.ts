import { clamp } from "../../math";

import type { SliderRange } from "./slider-range";

/** The step of an arrow key: 1° or 1 % (`RULE-interaction-colors`). */
const ARROW_STEP = 1;

/** The larger step of Page Up and Page Down: 10° or 10 % (`RULE-interaction-colors`). */
const PAGE_STEP = 10;

/** What each arrow and page key adds to a slider's value. */
const KEY_STEPS = new Map([
  ["ArrowDown", -ARROW_STEP],
  ["ArrowLeft", -ARROW_STEP],
  ["ArrowRight", ARROW_STEP],
  ["ArrowUp", ARROW_STEP],
  ["PageDown", -PAGE_STEP],
  ["PageUp", PAGE_STEP],
]);

/**
 * A slider's value after a key, as the WAI-ARIA slider answers it: Right and Up add one step, Left
 * and Down take one away, Page Up and Page Down ten, kept within the range; Home and End go to its
 * first and last values (`REF-WAI-APG-SLIDER`, Keyboard Interaction). A hue of 30° is 31° after
 * Right, 40° after Page Up, 0° after Home, 359° after End.
 *
 * @kind domain
 * @param key - the key pressed, as `KeyboardEvent.key` names it
 * @param value - the value before the key
 * @param range - the slider's first and last values
 * @returns the value after the key; `undefined` for a key the slider does not answer
 * @see REF-WAI-APG-SLIDER
 */
export function valueAfterSliderKey(
  key: string,
  value: number,
  range: SliderRange,
): number | undefined {
  if (key === "Home") {
    return range.minimum;
  }

  if (key === "End") {
    return range.maximum;
  }

  const step = KEY_STEPS.get(key);

  return step === undefined ? undefined : clamp(value + step, range.minimum, range.maximum);
}

import { PERCENT_SLIDER_RANGE } from "./percent-slider-range";
import { valueAfterSliderKey } from "./value-after-slider-key";

import type { PlaneValues } from "./plane-values-at";

/** The keys that move the plane's point up and down: its lightness or brightness. */
const LEVEL_KEYS = new Set(["ArrowDown", "ArrowUp"]);

/**
 * A picker's plane after a key, each axis answering as a WAI-ARIA slider (`valueAfterSliderKey`):
 * Up and Down change the lightness or brightness by 1 %; Left and Right change the saturation by
 * 1 %, Page Up and Page Down by 10 %, Home and End take it to 0 and 100 %
 * (`RULE-interaction-colors`; `REF-WAI-APG-SLIDER`, which has no two-dimensional pattern).
 *
 * @kind domain
 * @param key - the key pressed, as `KeyboardEvent.key` names it
 * @param values - the saturation and the lightness or brightness before the key
 * @returns the values after the key; `undefined` for a key the plane does not answer
 * @see REF-WAI-APG-SLIDER
 */
export function planeValuesAfterKey(key: string, values: PlaneValues): PlaneValues | undefined {
  const isLevelKey = LEVEL_KEYS.has(key);
  const moved = valueAfterSliderKey(
    key,
    isLevelKey ? values.level : values.saturation,
    PERCENT_SLIDER_RANGE,
  );

  if (moved === undefined) {
    return undefined;
  }

  return isLevelKey ? { ...values, level: moved } : { ...values, saturation: moved };
}

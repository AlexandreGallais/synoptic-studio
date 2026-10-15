import type { SliderRange } from "./slider-range";

/**
 * The values the hue slider holds, 0 to 359.9°: a hue of 360 is shown 0.0, so End goes to the
 * last hue shown (`docs/domain/interaction.md` §5; `DERIV-color-spaces`, step 9).
 */
export const HUE_SLIDER_RANGE: SliderRange = { maximum: 359.9, minimum: 0 };

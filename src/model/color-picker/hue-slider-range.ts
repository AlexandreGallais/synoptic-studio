import type { SliderRange } from "./slider-range";

/**
 * The values the hue slider holds, whole degrees from 0 to 359: a hue of 360 is shown 0, so End
 * goes to the last hue shown (`RULE-interaction-colors`; ADR-0058, decision 4).
 */
export const HUE_SLIDER_RANGE: SliderRange = { maximum: 359, minimum: 0 };

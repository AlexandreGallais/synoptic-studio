import { PERCENT } from "../../math";

import type { SliderRange } from "./slider-range";

/**
 * The values of a slider in percent, 0 to 100: the opacity slider, and the plane's saturation and
 * lightness or brightness (`RULE-interaction-colors`).
 */
export const PERCENT_SLIDER_RANGE: SliderRange = { maximum: PERCENT, minimum: 0 };

import { PERCENT } from "../../math";

import type { SliderRange } from "./slider-range";

/**
 * The values of a slider in percent, 0 to 100: the opacity slider, and the plane's saturation and
 * lightness or brightness (`docs/domain/interaction.md` §5).
 */
export const PERCENT_SLIDER_RANGE: SliderRange = { maximum: PERCENT, minimum: 0 };

import {
  DEGREES_PER_TURN,
  PERCENT_SLIDER_RANGE,
  REGULAR_POLYGON_CORNER_RANGE,
  TEXT_RANGES,
} from "../../src";
import { HISTORY_BOUND_FIELDS } from "../history-bound";
import { MAX_SIZE } from "../shape";

import type { OwnFieldRange } from "./own-field-range";

/** A range of whole numbers, a value beyond it taken at its bound. */
const WHOLE = { isTurn: false, isWhole: true };

/** Whole numbers without a bound: a coordinate. */
const UNBOUNDED: OwnFieldRange = { ...WHOLE, minimum: -Infinity, maximum: Infinity };

/** An angle in whole degrees, from 0 to 359, a value beyond going round: 370 is 10. */
const TURN: OwnFieldRange = { isTurn: true, isWhole: true, minimum: 0, maximum: DEGREES_PER_TURN };

/** An opacity, in whole percent from 0 to 100. */
const OPACITY: OwnFieldRange = {
  ...WHOLE,
  minimum: TEXT_RANGES.opacity.first,
  maximum: TEXT_RANGES.opacity.last,
};

/** A letter or word spacing, in whole user units from −100 000 to 100 000. */
const SPACING: OwnFieldRange = {
  ...WHOLE,
  minimum: TEXT_RANGES.spacing.first,
  maximum: TEXT_RANGES.spacing.last,
};

/** A radius, in whole user units from 0 to the interface's largest size (Q20). */
const RADIUS: OwnFieldRange = { ...WHOLE, minimum: 0, maximum: MAX_SIZE };

/** A value of the color picker in whole percent, a saturation, a lightness or a brightness. */
const PERCENT: OwnFieldRange = {
  ...WHOLE,
  minimum: PERCENT_SLIDER_RANGE.minimum,
  maximum: PERCENT_SLIDER_RANGE.maximum,
};

/**
 * The values each field acting as a number field of its own takes, by its id (US-087, AUD-046),
 * from the constants of what it sets, so that the page writes no bound of its own: a value beyond
 * a range is taken at its bound, an angle's, the Angle's and the picker's hue, going round; every
 * field holds whole numbers, its value always rounded (AUD-047), but the Zoom's percentage, from
 * 1 %, its maximum the view's own (`boundedScale`).
 */
export const OWN_FIELD_RANGES: ReadonlyMap<string, OwnFieldRange> = new Map([
  ["history-memory", { ...WHOLE, minimum: 0, maximum: HISTORY_BOUND_FIELDS.size.maximum }],
  ["history-steps", { ...WHOLE, minimum: 0, maximum: HISTORY_BOUND_FIELDS.steps.maximum }],
  ["zoom-field", { isTurn: false, isWhole: false, minimum: 1, maximum: Infinity }],
  [
    "corners",
    {
      ...WHOLE,
      minimum: REGULAR_POLYGON_CORNER_RANGE.min,
      maximum: REGULAR_POLYGON_CORNER_RANGE.max,
    },
  ],
  ["text-size", { ...WHOLE, minimum: TEXT_RANGES.size.first, maximum: TEXT_RANGES.size.last }],
  ["text-opacity", OPACITY],
  ["text-letter-spacing", SPACING],
  ["text-word-spacing", SPACING],
  ["text-underline-opacity", OPACITY],
  ["text-strike-opacity", OPACITY],
  [
    "text-maximum-lines",
    { ...WHOLE, minimum: TEXT_RANGES.maximumLines.first, maximum: TEXT_RANGES.maximumLines.last },
  ],
  [
    "text-line-height",
    { ...WHOLE, minimum: TEXT_RANGES.lineHeight.first, maximum: TEXT_RANGES.lineHeight.last },
  ],
  ["text-background-opacity", OPACITY],
  ["color-hue-value", TURN],
  ["color-saturation-value", PERCENT],
  ["color-level-value", PERCENT],
  ["angle", TURN],
  ["radius", RADIUS],
  ["point-radius", RADIUS],
  ["handle-before-x", UNBOUNDED],
  ["handle-before-y", UNBOUNDED],
  ["handle-after-x", UNBOUNDED],
  ["handle-after-y", UNBOUNDED],
  ["point-x", UNBOUNDED],
  ["point-y", UNBOUNDED],
]);

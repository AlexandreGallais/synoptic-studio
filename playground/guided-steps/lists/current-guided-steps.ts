import { GUIDED_STEPS_F05, GUIDED_STEPS_F14, GUIDED_STEPS_F15, GUIDED_STEPS_F16 } from "../epic-15";

import type { GuidedStep } from "../guided-step";

/**
 * Steps of the guided test of the features being validated, replaced at each validation: E15's
 * four features validated at once (VAL-017), F05's, F14's, F16's and F15's steps in a row. Empty
 * between features: the published playground then shows no guided test.
 */
export const CURRENT_GUIDED_STEPS: readonly GuidedStep[] = [
  ...GUIDED_STEPS_F05,
  ...GUIDED_STEPS_F14,
  ...GUIDED_STEPS_F16,
  ...GUIDED_STEPS_F15,
];

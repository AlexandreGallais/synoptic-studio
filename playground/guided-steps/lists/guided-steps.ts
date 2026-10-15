import { GUIDED_STEPS_OF_EPIC_1 } from "../epic-1";
import { GUIDED_STEPS_F10, GUIDED_STEPS_F12 } from "../epic-14";
import { GUIDED_STEPS_F05, GUIDED_STEPS_F14 } from "../epic-15";

import type { GuidedStep } from "../guided-step";

/**
 * The steps of the Product Owner test cards, shown one at a time: E01's, then F12's and F10's
 * (E14), then F05's and F14's (E15).
 */
export const GUIDED_STEPS: readonly GuidedStep[] = [
  ...GUIDED_STEPS_OF_EPIC_1,
  ...GUIDED_STEPS_F12,
  ...GUIDED_STEPS_F10,
  ...GUIDED_STEPS_F05,
  ...GUIDED_STEPS_F14,
];

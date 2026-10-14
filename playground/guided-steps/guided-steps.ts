import { GUIDED_STEPS_F10 } from "./guided-steps-f10";
import { GUIDED_STEPS_F12 } from "./guided-steps-f12";
import { GUIDED_STEPS_OF_EPIC_1 } from "./guided-steps-of-epic-1";

import type { GuidedStep } from "./guided-step";

/** The steps of the Product Owner test cards, shown one at a time: E01's, then F12's and F10's (E14). */
export const GUIDED_STEPS: readonly GuidedStep[] = [
  ...GUIDED_STEPS_OF_EPIC_1,
  ...GUIDED_STEPS_F12,
  ...GUIDED_STEPS_F10,
];

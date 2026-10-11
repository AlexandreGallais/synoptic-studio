import { GUIDED_STEPS_F01 } from "./guided-steps-f01";
import { GUIDED_STEPS_F02 } from "./guided-steps-f02";
import { GUIDED_STEPS_F03 } from "./guided-steps-f03";
import { GUIDED_STEPS_F07 } from "./guided-steps-f07";

import type { GuidedStep } from "./guided-step";

/** The steps of the Product Owner test cards, shown one at a time: F01's, F02's, F07's, then F03's. */
export const GUIDED_STEPS: readonly GuidedStep[] = [
  ...GUIDED_STEPS_F01,
  ...GUIDED_STEPS_F02,
  ...GUIDED_STEPS_F07,
  ...GUIDED_STEPS_F03,
];

import { GUIDED_STEPS_F01 } from "./guided-steps-f01";
import { GUIDED_STEPS_F02 } from "./guided-steps-f02";
import { GUIDED_STEPS_F03 } from "./guided-steps-f03";
import { GUIDED_STEPS_F04 } from "./guided-steps-f04";
import { GUIDED_STEPS_F07 } from "./guided-steps-f07";
import { GUIDED_STEPS_F08 } from "./guided-steps-f08";
import { GUIDED_STEPS_F09 } from "./guided-steps-f09";

import type { GuidedStep } from "./guided-step";

/** The steps of the Product Owner test cards, shown one at a time: F01's, F02's, F07's, F03's, then F09's. */
export const GUIDED_STEPS: readonly GuidedStep[] = [
  ...GUIDED_STEPS_F01,
  ...GUIDED_STEPS_F02,
  ...GUIDED_STEPS_F07,
  ...GUIDED_STEPS_F03,
  ...GUIDED_STEPS_F09,
  ...GUIDED_STEPS_F08,
  ...GUIDED_STEPS_F04,
];

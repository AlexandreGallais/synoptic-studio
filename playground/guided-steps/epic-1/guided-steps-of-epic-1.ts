import { GUIDED_STEPS_EPIC_1 } from "./guided-steps-epic-1";
import { GUIDED_STEPS_F01 } from "./guided-steps-f01";
import { GUIDED_STEPS_F02 } from "./guided-steps-f02";
import { GUIDED_STEPS_F03 } from "./guided-steps-f03";
import { GUIDED_STEPS_F04 } from "./guided-steps-f04";
import { GUIDED_STEPS_F06 } from "./guided-steps-f06";
import { GUIDED_STEPS_F07 } from "./guided-steps-f07";
import { GUIDED_STEPS_F08 } from "./guided-steps-f08";
import { GUIDED_STEPS_F09 } from "./guided-steps-f09";

import type { GuidedStep } from "../guided-step";

/** The steps of the epic E01's test cards: F01's, F02's, F07's, F03's, F09's, F08's, F04's, F06's, then the user test of the epic. */
export const GUIDED_STEPS_OF_EPIC_1: readonly GuidedStep[] = [
  ...GUIDED_STEPS_F01,
  ...GUIDED_STEPS_F02,
  ...GUIDED_STEPS_F07,
  ...GUIDED_STEPS_F03,
  ...GUIDED_STEPS_F09,
  ...GUIDED_STEPS_F08,
  ...GUIDED_STEPS_F04,
  ...GUIDED_STEPS_F06,
  ...GUIDED_STEPS_EPIC_1,
];

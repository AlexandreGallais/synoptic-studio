import { GUIDED_STEPS_F01 } from "./guided-steps-f01";
import { GUIDED_STEPS_F02 } from "./guided-steps-f02";

import type { GuidedStep } from "./guided-step";

/** The steps of the Product Owner test cards, shown one at a time: F01's, then F02's. */
export const GUIDED_STEPS: readonly GuidedStep[] = [...GUIDED_STEPS_F01, ...GUIDED_STEPS_F02];

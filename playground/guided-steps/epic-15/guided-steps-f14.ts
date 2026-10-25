import { GUIDED_STEPS_F14_DECISIONS } from "./guided-steps-f14-decisions";
import { GUIDED_STEPS_F14_OPTIONS } from "./guided-steps-f14-options";

import type { GuidedStep } from "../guided-step";

/** The F14 steps: its options, then its decisions to confirm and back to the rectangle. */
export const GUIDED_STEPS_F14: readonly GuidedStep[] = [
  ...GUIDED_STEPS_F14_OPTIONS,
  ...GUIDED_STEPS_F14_DECISIONS,
];

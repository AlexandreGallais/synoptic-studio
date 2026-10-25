import { GUIDED_STEPS_F16_DECISIONS } from "./guided-steps-f16-decisions";
import { GUIDED_STEPS_F16_OPTIONS } from "./guided-steps-f16-options";

import type { GuidedStep } from "../guided-step";

/** The F16 steps: its options, then its decisions to confirm and back to the rectangle. */
export const GUIDED_STEPS_F16: readonly GuidedStep[] = [
  ...GUIDED_STEPS_F16_OPTIONS,
  ...GUIDED_STEPS_F16_DECISIONS,
];

import { GUIDED_STEPS_F05_DECISIONS } from "./guided-steps-f05-decisions";
import { GUIDED_STEPS_F05_OPTIONS } from "./guided-steps-f05-options";

import type { GuidedStep } from "../guided-step";

/** The F05 steps (VAL-014): its options, then its decisions to confirm and the browser checks. */
export const GUIDED_STEPS_F05: readonly GuidedStep[] = [
  ...GUIDED_STEPS_F05_OPTIONS,
  ...GUIDED_STEPS_F05_DECISIONS,
];

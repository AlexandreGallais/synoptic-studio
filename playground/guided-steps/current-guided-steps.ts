import { GUIDED_STEPS_F10 } from "./guided-steps-f10";

import type { GuidedStep } from "./guided-step";

/**
 * Steps of the guided test of the feature being validated: a few short steps, replaced at each
 * validation. Empty between features: the published playground then shows no guided test.
 */
export const CURRENT_GUIDED_STEPS: readonly GuidedStep[] = GUIDED_STEPS_F10;

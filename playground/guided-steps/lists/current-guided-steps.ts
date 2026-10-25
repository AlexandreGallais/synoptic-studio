import type { GuidedStep } from "../guided-step";

/**
 * Steps of the guided test of the features being validated. Empty since E15's validation: a
 * feature is now validated on its epic's checklist (`?checklist=E15`); the steps stay in
 * the epic's list (`?guide=epic`). Empty, the published playground shows no guided test.
 */
export const CURRENT_GUIDED_STEPS: readonly GuidedStep[] = [];

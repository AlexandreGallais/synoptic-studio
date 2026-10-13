import { CURRENT_GUIDED_STEPS, GUIDED_STEPS } from "../guided-steps";

import type { GuidedStep } from "../guided-steps";

/**
 * Guided test asked by the page address: `?guide=epic` gives every step of the epic, for its
 * review; otherwise the steps of the feature being validated.
 *
 * @kind procedure
 * @param document - page document
 * @returns the steps to show, possibly none
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
export function readGuidedTestSteps(document: Document): readonly GuidedStep[] {
  const query = new URLSearchParams(document.location.search);

  return query.get("guide") === "epic" ? GUIDED_STEPS : CURRENT_GUIDED_STEPS;
}

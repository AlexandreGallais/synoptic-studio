import { createTextStepValues } from "./create-text-step-values";

import type { GuidedStep, GuidedText } from "../guided-step";

/** The text of F16's steps: "Pump P-101" in Inter 32, fitting its content, opaque. */
const COLOR_TEXT = { content: "Pump P-101" } as const;

/**
 * The values of a step of F16: "Pump P-101" in Inter 32 at (0, 0), fitting its content, changed
 * by a few fields, its color and opacity, its background's among them.
 *
 * @kind procedure
 * @param fields - the fields that differ from that text
 * @returns the values Next types
 * @see docs/backlog/stories/E15-F16-VAL-016-validate-f16.md
 */
export function createColorStepValues(fields: Partial<GuidedText>): GuidedStep["values"] {
  return createTextStepValues({ ...COLOR_TEXT, ...fields });
}

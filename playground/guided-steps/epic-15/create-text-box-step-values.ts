import { createTextStepValues } from "./create-text-step-values";

import type { GuidedStep, GuidedText } from "../guided-step";

/** The text of F14's steps: "Hello World" in Inter 16, fitting its content, its box 200 × 100. */
const BOX_TEXT = { boxHeight: 100, boxWidth: 200, content: "Hello World", size: 16 } as const;

/**
 * The values of a step of F14: "Hello World" in Inter 16 at (0, 0), fitting its content, its box
 * 200 × 100 as the page opens it, changed by a few fields.
 *
 * @kind procedure
 * @param fields - the fields that differ from that text
 * @returns the values Next types
 * @see VAL-015
 */
export function createTextBoxStepValues(fields: Partial<GuidedText>): GuidedStep["values"] {
  return createTextStepValues({ ...BOX_TEXT, ...fields });
}

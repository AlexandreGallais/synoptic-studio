import { OPENING_TEXT } from "../guided-step";

import type { GuidedStep, GuidedText } from "../guided-step";

/** The values of a step of F05: the Text tool, the sharp rectangle 120 × 80 behind it. */
const RECTANGLE = {
  anchor: "mid mid",
  corners: 6,
  height: 80,
  radius: 0,
  shape: "text",
  width: 120,
} as const;

/**
 * The values of a step of F05: the text the page opens with, changed by a few fields.
 *
 * @kind procedure
 * @param fields - the fields that differ from the opening text
 * @returns the values Next types
 * @see VAL-014
 */
export function createTextStepValues(fields: Partial<GuidedText>): GuidedStep["values"] {
  return { ...RECTANGLE, text: { ...OPENING_TEXT, ...fields } };
}

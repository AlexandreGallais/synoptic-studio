import type { GuidedStep, GuidedText } from "../guided-step";

/** The text the page opens with: "Pump P-101" over "Running", Inter 32, regular, at (0, 0). */
const TEXT: GuidedText = {
  alignment: "left",
  angle: 0,
  color: "#1e293b",
  content: "Pump P-101\nRunning",
  direction: "ltr",
  font: "Inter",
  isItalic: false,
  isStruckThrough: false,
  isUnderlined: false,
  letterSpacing: 0,
  opacity: 100,
  size: 32,
  textCase: "none",
  weight: 400,
  wordSpacing: 0,
  x: 0,
  y: 0,
};

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
 * @see docs/backlog/stories/E15-F05-VAL-014-validate-f05.md
 */
export function createTextStepValues(fields: Partial<GuidedText>): GuidedStep["values"] {
  return { ...RECTANGLE, text: { ...TEXT, ...fields } };
}

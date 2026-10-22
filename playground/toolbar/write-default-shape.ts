import { writeStepValues } from "../guided-test";

/** The shape the page opens with, as `index.html` sets it. */
const DEFAULT_SHAPE = {
  explanation: "",
  look: "",
  title: "Reset",
  values: { anchor: "mid mid", corners: 6, height: 80, radius: 10, shape: "rectangle", width: 120 },
};

/**
 * Puts back the shape the page opens with, alone: a rectangle 120 × 80, radius 10, straight, at
 * (0, 0), no free shape kept, no point selected, the shapes placed and the settings each kind kept
 * forgotten, the Text tool's fields the page's (F35).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
export function writeDefaultShape(document: Document): void {
  writeStepValues(document, DEFAULT_SHAPE);
}

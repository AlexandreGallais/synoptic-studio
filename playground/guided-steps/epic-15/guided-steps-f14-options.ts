import { createTextBoxStepValues } from "./create-text-box-step-values";

import type { GuidedStep } from "../guided-step";

/** "Pump P-101 is running" in a box 50 wide: a word too long, "running", 57.53 wide. */
const PUMP_50 = { boxWidth: 50, content: "Pump P-101 is running", isInBox: true } as const;

/** The same with "running" moved whole. */
const MOVED = { ...PUMP_50, longWord: "move" } as const;

/** "Pump P-101 is running" in a box 110 wide: "Pump P-101 is" over "running". */
const PUMP_110 = { ...PUMP_50, boxWidth: 110 } as const;

/** "Pump P-101" over "Running", fitting its content. */
const TWO_LINES = { content: "Pump P-101\nRunning" } as const;

/**
 * The F14 options to try (VAL-015), one per step: Fit content or Box, the box's size, long words,
 * the maximum number of lines, the ellipsis, the vertical alignment, the line height, the trim,
 * the balance, the background, the turn about the box, values refused and back to F05.
 */
export const GUIDED_STEPS_F14_OPTIONS: readonly GuidedStep[] = [
  {
    explanation: "Fit: Box.",
    look: "The box's fields appear; “Hello” over “World”; the dashed frame is the box, 50 × 100.",
    title: "F14 · Fit content or Box",
    values: createTextBoxStepValues({ boxWidth: 50 }),
  },
  {
    explanation: "Width 200; then Height 10.",
    look: "“Hello World” on one line; at Height 10 it stays drawn, past the box's bottom.",
    title: "F14 · Width and height",
    values: createTextBoxStepValues({ boxWidth: 50, isInBox: true }),
  },
  {
    explanation: "Nothing to type: Long words Break.",
    look: "Five lines: Pump, P-101, is, runnin, g; no hyphen; the box, 100 tall, holds them (96.8).",
    title: "F14 · Long word broken",
    values: createTextBoxStepValues(PUMP_50),
  },
  {
    explanation: "Long words: Move whole.",
    look: "Four lines: Pump, P-101, is, “runn…” (48.2 wide).",
    title: "F14 · Long word moved whole",
    values: createTextBoxStepValues(PUMP_50),
  },
  {
    explanation: "Untick Ellipsis; tick it again.",
    look: "“running” whole (57.5), past the box's right edge at 50; then “runn…” again.",
    title: "F14 · Ellipsis",
    values: createTextBoxStepValues(MOVED),
  },
  {
    explanation: "Max lines 1; then 0.",
    look: "“Hell…” alone (42.8): “Hello…” (52.4) is wider than 50; then “Hello” over “World”.",
    title: "F14 · Max lines",
    values: createTextBoxStepValues({ boxWidth: 50, isInBox: true }),
  },
  {
    explanation: "Vertical alignment Middle, then Bottom.",
    look: "The four lines (77.4 tall) move down 11.3, then 22.6: the last one's bottom on the box's.",
    title: "F14 · Vertical alignment",
    values: createTextBoxStepValues(MOVED),
  },
  {
    explanation: "Alignment Center, then Right.",
    look: "“Hello World” (87.55) moves across the box 200: x 56.22, then 112.45.",
    title: "F14 · Alignment across the box",
    values: createTextBoxStepValues({ isInBox: true }),
  },
  {
    explanation: "Line height 24; then 0.",
    look: "Baselines 24 apart, the first at 17.82; then 19.36 apart again, the first at 15.5.",
    title: "F14 · Line height",
    values: createTextBoxStepValues(MOVED),
  },
  {
    explanation: "Tick Trim.",
    look: "The lines move up: the first capitals touch the box's top, its baseline 11.64 below it.",
    title: "F14 · Trim",
    values: createTextBoxStepValues(MOVED),
  },
  {
    explanation: "Wrap: Balance.",
    look: "“Pump P-101 is” over “running” becomes “Pump P-101” over “is running”.",
    title: "F14 · Balance",
    values: createTextBoxStepValues(PUMP_110),
  },
  {
    explanation: "Tick Background.",
    look: "A yellow band behind each line, 87.95 and 61.80 long, 19.36 tall, the two touching.",
    title: "F14 · Background",
    values: createTextBoxStepValues(TWO_LINES),
  },
  {
    explanation: "Background color #22c55e; Background opacity 40.",
    look: "Light green bands under the letters; SVG output: one path, fill-opacity 0.4.",
    title: "F14 · Background color and opacity",
    values: createTextBoxStepValues({ ...TWO_LINES, hasBackground: true }),
  },
  {
    explanation: "Angle 90.",
    look: "Text and box turn together about the box's middle: rotate(90 55 50).",
    title: "F14 · Turned about the box",
    values: createTextBoxStepValues(PUMP_110),
  },
  {
    explanation: "Width 100001; then empty; then 110. Background opacity 101; then 100.",
    look: "Each refused: drawing gone, message, fields in red; back when valid.",
    title: "F14 · Refused values",
    values: createTextBoxStepValues(PUMP_110),
  },
  {
    explanation: "Fit: Fit content.",
    look: "The box's fields go; one line, 166.8 wide, its frame hugging it, as in F05 (F14.AC6).",
    title: "F14 · Back to F05",
    values: createTextBoxStepValues(PUMP_110),
  },
];

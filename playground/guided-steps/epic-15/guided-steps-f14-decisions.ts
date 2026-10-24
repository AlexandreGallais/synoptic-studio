import { createTextBoxStepValues } from "./create-text-box-step-values";

import type { GuidedStep } from "../guided-step";

/** A text in a box 60 wide, at most one line. */
const ONE_LINE = { boxWidth: 60, isInBox: true, maximumLines: 1 } as const;

/** "Pump P-101 is running" in a box. */
const PUMP = { content: "Pump P-101 is running", isInBox: true } as const;

/** "Hello" over "World" with a background. */
const BANDS = { content: "Hello\nWorld", hasBackground: true } as const;

/**
 * The F14 decisions to confirm (VAL-015), one per row of the story's table with its example, in
 * Inter 16 as the playground measures it, then back to the rectangle (F14.AC6).
 */
export const GUIDED_STEPS_F14_DECISIONS: readonly GuidedStep[] = [
  {
    explanation: "Width 100001, Enter; then 60, Enter.",
    look: "Capped at 100 000, then “Hello” over “World” in the box 60 × 40: from 0 to 100 000.",
    title: "F14 · Box from 0 to 100 000 (decision 1)",
    values: createTextBoxStepValues({ boxHeight: 40, boxWidth: 60, isInBox: true }),
  },
  {
    explanation: "Nothing to type: a space before “Hello”.",
    look: "“ Hello” over “World”: the opening space stays with its word, no line of spaces.",
    title: "F14 · Opening spaces (decision 2)",
    values: createTextBoxStepValues({ boxWidth: 60, content: " Hello World", isInBox: true }),
  },
  {
    explanation: "Untick Ellipsis.",
    look: "“runn…” (48.2) on its own line; then “running” whole (57.5), past the box's edge.",
    title: "F14 · Long word kept whole (decisions 3 and 4)",
    values: createTextBoxStepValues({ boxWidth: 50, content: "running", isInBox: true }),
  },
  {
    explanation: "Nothing to type: “Hello”, a no-break space, “World”, the ellipsis off.",
    look: "One line “Hello World” (82.7), past the box 45 wide: no break at a no-break space.",
    title: "F14 · No-break space (decision 5)",
    values: createTextBoxStepValues({
      boxWidth: 45,
      content: "Hello\u{A0}World",
      hasEllipsis: false,
      isInBox: true,
    }),
  },
  {
    explanation: "Nothing to type: a box 50 × 5. Then Max lines 100001.",
    look: "“Hell…”: one line at least, though 19.36 is taller than 5; then refused.",
    title: "F14 · One line at least (decision 6)",
    values: createTextBoxStepValues({ boxHeight: 5, boxWidth: 50, isInBox: true }),
  },
  {
    explanation: "Untick Ellipsis.",
    look: "“Hello…”; then “Hello”: the cut text simply stops.",
    title: "F14 · Ellipsis on or off (decision 7)",
    values: createTextBoxStepValues(ONE_LINE),
  },
  {
    explanation: "Width 50.",
    look: "“Hello…” (52.4) in 60; “Hell…” (42.8) in 50: characters go, not the whole word.",
    title: "F14 · Ellipsis removes characters (decision 8)",
    values: createTextBoxStepValues(ONE_LINE),
  },
  {
    explanation: "Nothing to type: “W a” in a box 5 wide.",
    look: "W over a, each wider than 5, neither with an ellipsis.",
    title: "F14 · One character too wide (decision 9)",
    values: createTextBoxStepValues({ boxWidth: 5, content: "W a", isInBox: true }),
  },
  {
    explanation: "Nothing to type: “Hello” and a line break, at most 1 line.",
    look: "“Hello”, no ellipsis: the empty line left out adds none.",
    title: "F14 · Empty lines left out (decision 10)",
    values: createTextBoxStepValues({ ...ONE_LINE, content: "Hello\n" }),
  },
  {
    explanation: "Nothing to type: a box 10 wide, at most 1 line.",
    look: "“H…”, 25.7 wide, past the box's edge. Keep it, draw only what fits, or nothing?",
    title: "F14 · Even “H…” too wide (decision 11)",
    values: createTextBoxStepValues({ ...ONE_LINE, boxWidth: 10 }),
  },
  {
    explanation: "Line height 30; then 0.",
    look: "Baseline (30 − 19.36) / 2 + 15.5 = 20.82 below the top; then 15.5 (automatic).",
    title: "F14 · Line height spread (decision 12)",
    values: createTextBoxStepValues({ content: "Hello" }),
  },
  {
    explanation: "Nothing to type: a box 110 wide; look for a Wrap field.",
    look: "“Pump P-101 is” over “running”, the automatic wrap; no Wrap field.",
    title: "F14 · Automatic wrap only (decisions 13 to 15)",
    values: createTextBoxStepValues({ ...PUMP, boxWidth: 110 }),
  },
  {
    explanation: "Line height 30.",
    look: "Lines 30 apart without a box (baselines 20.82, 50.82); the box's own fields stay hidden.",
    title: "F14 · Options of every text (decision 16)",
    values: createTextBoxStepValues({ content: "Hello\nWorld" }),
  },
  {
    explanation: "Tick Background; then Fit: Box.",
    look: "Yellow, opaque; Max 0, Ellipsis on, Top; Width and Height 88 × 20, the text rounded up (model box 0 × 0).",
    title: "F14 · Defaults (decision 17)",
    values: createTextBoxStepValues({}),
  },
  {
    explanation: "Nothing to type: “Hello” with a background.",
    look: "A band 19.36 tall: 15.5 above the baseline (ascender), 3.86 below (descender).",
    title: "F14 · Band from ascender to descender (decision 18)",
    values: createTextBoxStepValues({ content: "Hello", hasBackground: true }),
  },
  {
    explanation: "Tick Trim.",
    look: "Bands from 2.32 and 26.32, 4.64 apart; trimmed, the first from −3.86, above the top.",
    title: "F14 · Bands under a line height (decision 19)",
    values: createTextBoxStepValues({ ...BANDS, lineHeight: 24 }),
  },
  {
    explanation: "Nothing to type: line height 12, background opacity 40.",
    look: "The bands overlap by 7.36, the overlap no darker: one path, filled once.",
    title: "F14 · Overlapping bands (decision 20)",
    values: createTextBoxStepValues({ ...BANDS, backgroundOpacity: 40, lineHeight: 12 }),
  },
  {
    explanation: "Angle 90.",
    look: "rotate(90 30 20): the box's middle, not that of its lines (22.25, 19.36).",
    title: "F14 · Turned about the box's middle (decision 21)",
    values: createTextBoxStepValues({
      boxHeight: 40,
      boxWidth: 60,
      content: "Hello\nWorld",
      isInBox: true,
    }),
  },
  {
    explanation: "Nothing to type: “100”, U+202F, “%”; under it “100”, U+00A0, “%”.",
    look: "“100”, U+202F and “…”; under it “100…”: the cut never comes right after U+00A0.",
    title: "F14 · Other no-break characters (decision 22)",
    values: createTextBoxStepValues({
      boxWidth: 44,
      content: "100\u{202F}%\n100\u{A0}%",
      isInBox: true,
    }),
  },
  {
    explanation: "Letter spacing 10; then Width 160.",
    look: "The spacing moves nothing; at 160 the box's turned corner stays put, X 47, Y 61 its straight box (AUD-040).",
    title: "F14 · A turned box resized in place (US-038)",
    values: createTextBoxStepValues({
      angle: 23,
      boxHeight: 40,
      boxWidth: 120,
      content: "Pump P-101",
      isInBox: true,
      x: 47,
      y: 61,
    }),
  },
  {
    explanation: "Angle 90.",
    look: "Around the turned frame, an upright dashed box: Straight box x 10, y -10, width 40, height 60 (US-039).",
    title: "F14 · The straight box of a turned text (US-039)",
    values: createTextBoxStepValues({
      boxHeight: 40,
      boxWidth: 60,
      content: "Pump P-101",
      isInBox: true,
    }),
  },
  {
    explanation: "Rectangle.",
    look: "The sharp rectangle 120 × 80, as before; the text's fields go (F14.AC6).",
    title: "F14 · Back to the rectangle",
    values: createTextBoxStepValues({}),
  },
];

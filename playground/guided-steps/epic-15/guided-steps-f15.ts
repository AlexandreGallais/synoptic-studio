import { createTextBoxStepValues } from "./create-text-box-step-values";
import { createTextStepValues } from "./create-text-step-values";
import { GUIDED_STEPS_F15_FIRST_DECISIONS } from "./guided-steps-f15-first-decisions";
import { GUIDED_STEPS_F15_LAST_DECISIONS } from "./guided-steps-f15-last-decisions";

import type { GuidedStep } from "../guided-step";

/** "Pump P-101" in Inter 32, fitting its content. */
const PUMP = createTextStepValues({ content: "Pump P-101" });

/** "Pump P-101" in Arial 32 once Arial is imported, else in Inter. */
const ARIAL_PUMP = createTextStepValues({ content: "Pump P-101", font: "Arial" });

/** "Pump" in Inter 32, a line 38.71875 tall, in a box 50 × 20 too low for it, aligned in the middle. */
const TALL_LINE = {
  boxHeight: 20,
  boxWidth: 50,
  content: "Pump",
  isInBox: true,
  size: 32,
  verticalAlignment: "middle",
} as const;

/** "Overflowing words" in a box 50 × 100, its long word moved whole, one line at most. */
const ONE_WORD = {
  boxWidth: 50,
  content: "Overflowing words",
  hasEllipsis: false,
  isInBox: true,
  maximumLines: 1,
} as const;

/**
 * The F15 steps (VAL-017): its options — a font imported or installed, its weights, the text
 * measured with it, the fonts embedded, the HTML layer, the export — then the browser checks of the
 * HTML layer, its decisions to confirm, and back to the rectangle.
 */
export const GUIDED_STEPS_F15: readonly GuidedStep[] = [
  {
    explanation: "Open the Font list.",
    look: "Inter alone, and no field to type a name: only a font the editor measures is offered.",
    title: "F15 · The font list",
    values: PUMP,
  },
  {
    explanation: "Import a font: Arial.ttf (macOS: /System/Library/Fonts/Supplemental).",
    look: "“Arial 400 imported.”; Font Arial at once, the text redrawn in Arial, narrower.",
    title: "F15 · Import a font",
    values: PUMP,
  },
  {
    explanation: "Import a font: any other file renamed with .ttf (a .txt, a .png).",
    look: "“<its name> is not a TrueType or OpenType font file (.ttf, .otf): refused.”; Font unchanged.",
    title: "F15 · A file refused",
    values: ARIAL_PUMP,
  },
  {
    explanation: "Import a font: Arial Bold.ttf and Arial Italic.ttf together; open Weight.",
    look: "Weight offers 400 Regular and 700 Bold only; Bold draws 700, Italic the italic face.",
    title: "F15 · A family's weights",
    values: ARIAL_PUMP,
  },
  {
    explanation: "In Chrome: Installed fonts, allow; then Font Georgia.",
    look: "The computer's families after Arial; “Reading Georgia…”, then “Georgia: 4 faces read.”",
    title: "F15 · Installed fonts (Chrome)",
    values: ARIAL_PUMP,
  },
  {
    explanation: "In Firefox or Safari: Installed fonts.",
    look: "“This browser cannot list the installed fonts (Chrome and Edge can): use Import a font with the font's file.”",
    title: "F15 · Installed fonts elsewhere",
    values: PUMP,
  },
  {
    explanation: "Font Arial; then Font Inter; read the Contour column each time.",
    look: "Arial: width 82.3984375, height 18.3984375; Inter: 87.5546875 × 19.359375.",
    title: "F15 · Measured with the font chosen",
    values: createTextStepValues({ content: "Hello World", font: "Arial", size: 16 }),
  },
  {
    explanation: "Tick Embed fonts; then Font Inter.",
    look: "Arial: the SVG output opens with <defs><style>@font-face; Inter: “Inter 400: not embedded, its file not held.”",
    title: "F15 · Embed fonts",
    values: ARIAL_PUMP,
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "Ticked: the SVG output holds <switch>, a <foreignObject> then the lines; the drawing unchanged.",
    title: "F15 · HTML layer",
    values: ARIAL_PUMP,
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "Ticked: the Arabic letters joined by the browser; unticked: the library's lines, letters apart.",
    title: "F15 · Arabic in the HTML layer",
    values: createTextStepValues({ content: "مضخة" }),
  },
  {
    explanation: "Tick Embed fonts and HTML layer; Export SVG.",
    look: "drawing.svg saved; opened in the browser, the text in Arial without the dashed frame.",
    title: "F15 · Export SVG",
    values: ARIAL_PUMP,
  },
  {
    explanation: "Open the drawing.svg of the step before in Chrome, Firefox and Safari.",
    look: "“Pump P-101” in Arial in each: the browser's HTML layer (its letters can be selected).",
    title: "F15 · The export in three browsers",
    values: ARIAL_PUMP,
  },
  {
    explanation: "Open the same drawing.svg in Inkscape.",
    look: "The positioned lines; say whether Inkscape draws the embedded Arial (research note 0020).",
    title: "F15 · The export in Inkscape",
    values: ARIAL_PUMP,
  },
  {
    explanation: "Tick HTML layer; Vertical alignment Bottom; untick HTML layer.",
    look: "The line passes the box above and below, then above only, the same with the layer or without.",
    title: "F15 · Browser check 1, a line past the box",
    values: createTextBoxStepValues(TALL_LINE),
  },
  {
    explanation: "Tick HTML layer; then Width 0 and Height 40.",
    look: "No <foreignObject> in the SVG output either time: the positioned lines alone.",
    title: "F15 · Browser check 2, an empty box",
    values: createTextBoxStepValues({ boxHeight: 0, boxWidth: 80, isInBox: true }),
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "“Hello” at the same place both times, at the box's bottom above the empty line.",
    title: "F15 · Browser check 3, a line break at the bottom",
    values: createTextBoxStepValues({
      boxWidth: 100,
      content: "Hello\n",
      isInBox: true,
      verticalAlignment: "bottom",
    }),
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "“Overflowing” passes the box's right edge, “words” not shown, both times.",
    title: "F15 · Browser check 4, a word moved whole",
    values: createTextBoxStepValues(ONE_WORD),
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "The word ends with “…” inside the box, both times.",
    title: "F15 · Browser check 5, the same with the ellipsis",
    values: createTextBoxStepValues({ ...ONE_WORD, hasEllipsis: true }),
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "The j's hook and the f's top whole both times: nothing cut at the layer's edges.",
    title: "F15 · Browser check 6, nothing clipped",
    values: createTextStepValues({ content: "jumpy fjord", isItalic: true, size: 64 }),
  },
  {
    explanation: "Tick HTML layer; then Underline color #ff0000, Tab.",
    look: "“…” underlined and struck as in the lines; then the underline red, the “…” struck, not underlined.",
    title: "F15 · Browser check 7, the ellipsis's strokes",
    values: createTextBoxStepValues({
      ...ONE_WORD,
      hasEllipsis: true,
      isStruckThrough: true,
      isUnderlined: true,
    }),
  },
  ...GUIDED_STEPS_F15_FIRST_DECISIONS,
  ...GUIDED_STEPS_F15_LAST_DECISIONS,
  {
    explanation: "Rectangle.",
    look: "The sharp rectangle 120 × 80, as before; the text's fields and the drawing's options go.",
    title: "F15 · Back to the rectangle",
    values: PUMP,
  },
];

import { createTextBoxStepValues } from "./create-text-box-step-values";
import { createTextStepValues } from "./create-text-step-values";

import type { GuidedStep } from "../guided-step";

/** "Pump P-101" in Arial 16 once Arial is imported, else in Inter. */
const ARIAL_16 = createTextStepValues({ content: "Pump P-101", font: "Arial", size: 16 });

/** "Pump P-101 is running" in Arial 16 in a box 50 × 40: two lines at most. */
const BOX_50_40 = {
  boxHeight: 40,
  boxWidth: 50,
  content: "Pump P-101 is running",
  font: "Arial",
  isInBox: true,
  size: 16,
} as const;

/**
 * The F15 decisions to confirm (VAL-017), rows 19 to 36 of the story's table, the rules of EN-041,
 * US-036 and AUD-018, one step each with its example.
 */
export const GUIDED_STEPS_F15_LAST_DECISIONS: readonly GuidedStep[] = [
  {
    explanation: "Tick HTML layer; read the SVG output.",
    look: "The <foreignObject> holds a frame <div>, in it the content's <div>, one <span> per paragraph.",
    title: "F15 · The layer's elements (decision 19)",
    values: ARIAL_16,
  },
  {
    explanation: "Tick HTML layer; read the SVG output.",
    look: "line-height: 18.39844px: Arial 16's line height, 18.3984375, at 5 decimals.",
    title: "F15 · Numbers at 5 decimals (decision 20)",
    values: ARIAL_16,
  },
  {
    explanation: "Tick HTML layer; read the SVG output.",
    look: "color: #00000066: black at 40 % as one eight-digit color.",
    title: "F15 · Opacity in the layer's color (decision 21)",
    values: createTextStepValues({ color: "#000000", content: "Pump P-101", opacity: 40 }),
  },
  {
    explanation: "Tick HTML layer; untick Ellipsis; untick HTML layer.",
    look: "Two lines at most each time: the second ends with “…”, then is cut without it.",
    title: "F15 · At most the library's lines (decision 22)",
    values: createTextBoxStepValues(BOX_50_40),
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "The first baseline at the same height both times; the layer lowered by 3.65234 at the bottom.",
    title: "F15 · A trimmed text's first baseline (decision 23)",
    values: createTextBoxStepValues({
      ...BOX_50_40,
      content: "Pump",
      isTrimmed: true,
      verticalAlignment: "bottom",
    }),
  },
  {
    explanation: "Open Font; try to type monospace in it.",
    look: "Inter, then the fonts imported, then the installed ones; no name can be typed.",
    title: "F15 · The font list (decision 24)",
    values: ARIAL_16,
  },
  {
    explanation: "Import a font: Arial.ttf.",
    look: "“Arial 400 imported.”, Font Arial at once; a file refused says why.",
    title: "F15 · A font imported, chosen at once (decision 25)",
    values: createTextStepValues({ content: "Pump P-101", size: 16 }),
  },
  {
    explanation: "In Chrome: Installed fonts, allow; wait; then Font Georgia.",
    look: "The families listed at once, no file read; “Georgia: 4 faces read.” only once chosen.",
    title: "F15 · Installed families read when chosen (decision 26)",
    values: ARIAL_16,
  },
  {
    explanation: "Font Arial; then Bold.",
    look: "Weight 400, Arial having no 300; Bold 700 with Arial Bold imported, else 400 kept.",
    title: "F15 · The weight held (decision 27)",
    values: createTextStepValues({ content: "Pump P-101", size: 16, weight: 300 }),
  },
  {
    explanation: "Tick Embed fonts; then Italic.",
    look: "“Inter 400: not embedded, its file not held.”, then the same for “Inter 400 italic”.",
    title: "F15 · The faces left out, one line each (decision 28)",
    values: createTextStepValues({ content: "Pump P-101", size: 16 }),
  },
  {
    explanation: "Export SVG; open drawing.svg in the browser.",
    look: "drawing.svg: the text alone, without the dashed selection frame.",
    title: "F15 · Export SVG (decision 29)",
    values: ARIAL_16,
  },
  {
    explanation:
      "Import a font not installed on the computer (a .ttf downloaded from Google Fonts).",
    look: "The text drawn in that font's own outlines, not in a stand-in.",
    title: "F15 · A font given to the page (decision 30)",
    values: ARIAL_16,
  },
  {
    explanation: "In Chrome: Installed fonts; Font Georgia; then Font Helvetica.",
    look: "Arial kept until Georgia is read; Helvetica (a .ttc collection) gives the list back, said why.",
    title: "F15 · While a family is read (decision 31)",
    values: ARIAL_16,
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "The underline unbroken under the p both times, across its descender.",
    title: "F15 · The underline across descenders (decision 32)",
    values: createTextStepValues({ content: "Pump", font: "Arial", isUnderlined: true }),
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "The underline placed by the browser with the layer, by the font's post table without: close, maybe not equal.",
    title: "F15 · The underline from the font (decision 33)",
    values: createTextStepValues({ content: "Pump", font: "Arial", isUnderlined: true }),
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "With the layer the capitals' tops cut, 3.9375 of them; without it, whole.",
    title: "F15 · The layer clips its lines' ink (decision 34)",
    values: createTextBoxStepValues({
      ...BOX_50_40,
      boxWidth: 200,
      content: "Pump",
      lineHeight: 10,
    }),
  },
  {
    explanation: "Tick HTML layer; untick it.",
    look: "“Pump…”: with the layer the band stops before “…”; without it, it runs under it.",
    title: "F15 · No band under the layer's ellipsis (decision 35)",
    values: createTextBoxStepValues({
      ...BOX_50_40,
      boxWidth: 60,
      content: "Pump P-101",
      hasBackground: true,
      maximumLines: 1,
    }),
  },
  {
    explanation: "Tick HTML layer; read the SVG output.",
    look: "Two nested <div>, the <foreignObject> at y −0.26172, not 0: ADR-0053 amended once confirmed.",
    title: "F15 · Two div, the frame shifted (decision 36)",
    values: ARIAL_16,
  },
];

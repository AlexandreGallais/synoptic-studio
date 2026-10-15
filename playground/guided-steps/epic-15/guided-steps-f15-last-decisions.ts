import { createTextStepValues } from "./create-text-step-values";

import type { GuidedStep } from "../guided-step";

/** "Pump P-101" in Arial 16 once Arial is imported, else in Inter. */
const ARIAL_16 = createTextStepValues({ content: "Pump P-101", font: "Arial", size: 16 });

/**
 * The F15 decisions to confirm (VAL-017), rows 24 to 32 of the story's table, the rules of US-036
 * and AUD-018, one step each with its example; those of the HTML layer removed (US-044).
 */
export const GUIDED_STEPS_F15_LAST_DECISIONS: readonly GuidedStep[] = [
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
    explanation: "In Chrome: Installed fonts; Font Georgia.",
    look: "Arial kept until Georgia is read, then Georgia chosen; a family none of whose faces is read gives the list back to Arial, said why.",
    title: "F15 · While a family is read (decision 31)",
    values: ARIAL_16,
  },
  {
    explanation: "Nothing to type.",
    look: "The underline unbroken under the p, across its descender.",
    title: "F15 · The underline across descenders (decision 32)",
    values: createTextStepValues({ content: "Pump", font: "Arial", isUnderlined: true }),
  },
];

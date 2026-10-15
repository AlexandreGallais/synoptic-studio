import { createTextStepValues } from "./create-text-step-values";

import type { GuidedStep } from "../guided-step";

/** "Pump P-101" in Inter 32, fitting its content. */
const PUMP = createTextStepValues({ content: "Pump P-101" });

/** "Pump P-101" in Arial 16 once Arial is imported, else in Inter. */
const ARIAL_16 = createTextStepValues({ content: "Pump P-101", font: "Arial", size: 16 });

/** Said of a rule no file at hand shows: the tests prove it, the step only asks to confirm it. */
const TESTED = "Nothing to try: no file at hand shows it; the tests prove it.";

/**
 * The F15 decisions to confirm (VAL-017), rows 1 to 18 of the story's table, the rules of SP-018,
 * EN-039 and EN-040, one step each with its example; those of the HTML layer removed (US-044).
 */
export const GUIDED_STEPS_F15_FIRST_DECISIONS: readonly GuidedStep[] = [
  {
    explanation: "Nothing to type: read the Drawing options and the SVG output.",
    look: "Embed fonts unticked, the only option; the output starts with <g>: the drawing as before F15.",
    title: "F15 · Embed fonts off (decision 2)",
    values: ARIAL_16,
  },
  {
    explanation: "Import a font: Arial Bold Italic.ttf.",
    look: "“Arial 700 italic imported.”: family Arial, not “Arial Bold Italic”; a weight class of 950 would give 900.",
    title: "F15 · A face's family, weight and style (decision 3)",
    values: ARIAL_16,
  },
  {
    explanation: "Import a font: Arial.ttf, then Arial.ttf again; open Font and Weight.",
    look: "“Arial 400 imported.” twice; Arial once in Font, 400 once in Weight.",
    title: "F15 · A face read again replaces it (decision 4)",
    values: ARIAL_16,
  },
  {
    explanation: "Reload the page; open Font. Then import Arial.ttf and Arial Bold.ttf again.",
    look: "After the reload, Inter alone: a font imported is kept for the session only.",
    title: "F15 · Fonts kept for the session (decision 5)",
    values: ARIAL_16,
  },
  {
    explanation: "Tick Embed fonts; Export SVG.",
    look: "Arial (fsType 8) embedded: its @font-face in drawing.svg, no line under the options.",
    title: "F15 · Which faces may be embedded (decision 6)",
    values: ARIAL_16,
  },
  {
    explanation: "Tick Embed fonts.",
    look: "“Inter 400: not embedded, its file not held.”: its metrics ship, not its file.",
    title: "F15 · A face without its file (decision 7)",
    values: PUMP,
  },
  {
    explanation: "Tick Embed fonts; Bold, Export SVG; Bold again, Export SVG.",
    look: "One @font-face in each drawing.svg, the face drawn: 700, then 400; the whole file in base64, no local().",
    title: "F15 · Each face once, whole (decision 8)",
    values: ARIAL_16,
  },
  {
    explanation: "Export SVG; open drawing.svg in resvg or Batik if at hand.",
    look: "To choose: the Hebrew line drawn one width too far right there, kept as written or written from its left edge.",
    title: "F15 · Right to left in resvg and Batik (decision 10)",
    values: createTextStepValues({ content: "משאבה", direction: "rtl", font: "Arial", size: 16 }),
  },
  {
    explanation: TESTED,
    look: "A family named only in a Windows code page (Shift-JIS) is refused, as not a font.",
    title: "F15 · A family's name read in Unicode (decision 11)",
    values: ARIAL_16,
  },
  {
    explanation: TESTED,
    look: "A file naming no family (name ID 1 nor 16), or whose OS/2 table is too short, is refused.",
    title: "F15 · A file without a family refused (decision 12)",
    values: ARIAL_16,
  },
  {
    explanation: "Import a font: Georgia.ttf; open Font.",
    look: "Inter, Arial, Georgia: a new family after those kept; ARIAL imported after Arial stays Arial.",
    title: "F15 · First name kept, new family last (decision 13)",
    values: ARIAL_16,
  },
  {
    explanation: TESTED,
    look: "An OS/2 table of version 4 with fsType 10 (bits 2 and 8): embeddable, the least restrictive wins.",
    title: "F15 · Several permission bits (decision 14)",
    values: ARIAL_16,
  },
  {
    explanation:
      "Import a font: NotoSansJavanese-Regular.otf (macOS: /System/Library/Fonts/Supplemental); tick Embed fonts.",
    look: '“Noto Sans Javanese 400 imported.”; its rule\'s src data:font/otf;base64,…, format("opentype").',
    title: "F15 · An OpenType file with CFF outlines (decision 15)",
    values: ARIAL_16,
  },
  {
    explanation: TESTED,
    look: "Inter 400 held and 700 not, both used: no Inter embedded, “another face of its family used may not be embedded”.",
    title: "F15 · A family whole or not at all (decision 16)",
    values: ARIAL_16,
  },
  {
    explanation: TESTED,
    look: "A font named Monospace: left out, “its family is named like a generic family”.",
    title: "F15 · A generic name never embedded (decision 17)",
    values: ARIAL_16,
  },
  {
    explanation: TESTED,
    look: "A text in Helvetica measured with Inter: Inter left out, “reached only as the default font”.",
    title: "F15 · Only a family a text names (decision 18)",
    values: ARIAL_16,
  },
];

import { createTextStepValues } from "./create-text-step-values";

import type { GuidedStep } from "../guided-step";

/** "HH" in Inter 16, for the browser checks: 23.78125 wide, H being 1522 of 2048 units. */
const HH = { content: "HH", size: 16 } as const;

/**
 * The F05 decisions to confirm (VAL-014), one per step with its example, the questions of
 * AUD-015, two checks in the browser, then back to the rectangle (F05.AC6).
 */
export const GUIDED_STEPS_F05_DECISIONS: readonly GuidedStep[] = [
  {
    explanation: "Open the Font list.",
    look: "Inter, then the fonts imported or installed only: a font without metrics cannot be chosen (US-036).",
    title: "F05 · A font without metrics (decided, US-036)",
    values: createTextStepValues({}),
  },
  {
    explanation: "Font Inter, chosen in the list: no name is typed (US-036).",
    look: "SVG output font-family &quot;Inter&quot;, sans-serif: the name as the list holds it.",
    title: "F05 · Font name as listed (decided, US-036)",
    values: createTextStepValues({}),
  },
  {
    explanation: "Weight 100, Bold, Bold; then Weight 800, Bold, Bold.",
    look: "100 gives 700 pressed, then 100 back; 800 gives 700, then 800 back: the weight before.",
    title: "F05 · Bold button (decided, US-037)",
    values: createTextStepValues({}),
  },
  {
    explanation: "Letter spacing 100001; then −100000.",
    look: "Refused; then accepted, every line tightened past nothing: no frame.",
    title: "F05 · Spacing bounds ±100 000 (decision)",
    values: createTextStepValues({}),
  },
  {
    explanation: "Nothing to type: letter spacing −20, centered.",
    look: "“Pump P-101” is −4.09 wide, starts at 3.85 in a frame 3.61 wide (Pipeline, lines).",
    title: "F05 · A line narrower than nothing (decision)",
    values: createTextStepValues({ alignment: "center", letterSpacing: -20 }),
  },
  {
    explanation: "Underline.",
    look: "A stroke under “Running” (3.61 wide) only: none under a line 0 wide or less.",
    title: "F05 · No stroke under an empty line (decision)",
    values: createTextStepValues({ alignment: "center", letterSpacing: -20 }),
  },
  {
    explanation: "Nothing to type: “Pump”, a line separator (U+2028), “P-101”.",
    look: "One line, the separator a 21-wide box: only LF, CR and CR LF break a line (AUD Q2).",
    title: "F05 · Line breaks (decision and question)",
    values: createTextStepValues({ content: "Pump\u{2028}P-101" }),
  },
  {
    explanation: "Nothing to type: € − → ≤ ™ ș ț Ω Ж 中.",
    look: "Each in its Inter width; 中, outside Latin, Greek, Cyrillic, Hebrew and symbols, a 21 box.",
    title: "F05 · Characters measured (decision)",
    values: createTextStepValues({ content: "€ − → ≤ ™ ș ț Ω Ж 中" }),
  },
  {
    explanation: "Pipeline: block.",
    look: "Height 77.4375 = 2 × 38.71875: Inter has no line gap; Arial 16 adds 0.52 after each line.",
    title: "F05 · Where the line gap sits (question)",
    values: createTextStepValues({}),
  },
  {
    explanation: "Nothing to type: a damaged font of line height 0 or less cannot be shown here.",
    look: "Today it is kept as read: lines on top of each other. Refuse it, 0, or as read?",
    title: "F05 · A font of line height 0 (question)",
    values: createTextStepValues({}),
  },
  {
    explanation: "Nothing to type: “To” over “T o”.",
    look: "T and o 2.5 closer in “To” (kern −160 units); kerning read for all scripts at once.",
    title: "F05 · Kerning across scripts (question)",
    values: createTextStepValues({ content: "To\nT o" }),
  },
  {
    explanation:
      "Console (F12): $('#canvas text').getComputedTextLength(); Chrome, Firefox, Safari.",
    look: "33.78125: spacing between the letters only, as measured; 43.78125: also after the last.",
    title: "F05 · Browser: letter spacing after the last letter",
    values: createTextStepValues({ ...HH, letterSpacing: 10 }),
  },
  {
    explanation:
      "Console: $('#canvas text').setAttribute('text-decoration', 'underline'); 3 browsers.",
    look: "Native line on ours (top edge reading), or 6.84 lower (middle reading).",
    title: "F05 · Browser: our underline against the native one",
    values: createTextStepValues({ ...HH, isUnderlined: true, opacity: 50, size: 200 }),
  },
  {
    explanation: "Letter spacing 1, 2, … up to 10; then a second line; then Size 32.",
    look: "The P's turned corner stays put, the text grows along its line; X 47, Y 61 show that corner throughout (US-054).",
    title: "F05 · A turned text grows in place (US-038)",
    values: createTextStepValues({ angle: 23, content: "Pump P-101", size: 16, x: 47, y: 61 }),
  },
  {
    explanation: "Rectangle.",
    look: "The sharp rectangle 120 × 80, as before; the text's fields go, the shape's come back.",
    title: "F05 · Back to the rectangle",
    values: createTextStepValues({}),
  },
];

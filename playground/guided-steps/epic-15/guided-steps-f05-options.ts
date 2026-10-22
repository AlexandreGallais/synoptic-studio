import { createTextStepValues } from "./create-text-step-values";

import type { GuidedStep } from "../guided-step";

/**
 * The F05 options to try (VAL-014), one per step: every field of the Text tool, turning, values
 * refused, and each tool's own placement.
 */
export const GUIDED_STEPS_F05_OPTIONS: readonly GuidedStep[] = [
  {
    explanation: "Font Book (macOS): Inter 4.1 installed? If not, install it, then Next.",
    look: "The dashed frame hugs “Pump P-101” over “Running”; without Inter it does not.",
    title: "F05 · Inter installed",
    values: createTextStepValues({}),
  },
  {
    explanation: "Content: add a line break and “Tank T-1”; then end with a line break only.",
    look: "Three lines, frame 116.15625 tall (Pipeline, block); an empty last line counts too.",
    title: "F05 · Content and lines",
    values: createTextStepValues({}),
  },
  {
    explanation: "Empty Content.",
    look: "Nothing drawn, no frame, no message.",
    title: "F05 · Empty text",
    values: createTextStepValues({}),
  },
  {
    explanation: "Size 48; Pipeline.",
    look: "Text and frame grow: 263.85938 × 116.15625; first tspan y 46.5 (1984 × 48 / 2048).",
    title: "F05 · Size",
    values: createTextStepValues({}),
  },
  {
    explanation: "Weight 100, then 900.",
    look: "Thin, then black; Bold released at both, pressed at 700 only.",
    title: "F05 · Weight",
    values: createTextStepValues({}),
  },
  {
    explanation: "Italic; Italic again.",
    look: "Slanted, then upright.",
    title: "F05 · Italic",
    values: createTextStepValues({}),
  },
  {
    explanation: "Color #dc2626; Opacity 40, then 0.",
    look: "Red; fill-opacity 0.4 in SVG output; at 0 the text goes, the frame stays.",
    title: "F05 · Color and opacity",
    values: createTextStepValues({}),
  },
  {
    explanation: "Letter spacing 10.",
    look: "Frame 175.90625 → 265.90625 wide: 9 gaps × 10 in “Pump P-101”, none after the end.",
    title: "F05 · Letter spacing",
    values: createTextStepValues({}),
  },
  {
    explanation: "Word spacing 20.",
    look: "Only the space of “Pump P-101” widens: frame 195.90625; “Running” unchanged.",
    title: "F05 · Word spacing",
    values: createTextStepValues({}),
  },
  {
    explanation: "Underline; then untick it and tick Strike-through.",
    look: "One stroke under each line, as long as it, in the text's color; then one through it.",
    title: "F05 · Underline and strike-through",
    values: createTextStepValues({}),
  },
  {
    explanation: "Case: each of the nine, one after the other.",
    look: "As domain Q36 says: camel pumpP101IsRunning, kebab pump-p-101-is-running, and so on.",
    title: "F05 · Case (decision Q36)",
    values: createTextStepValues({ content: "pump P-101 is running" }),
  },
  {
    explanation: "Alignment Center, then Right.",
    look: "“Running” moves to x 26.14844, then 52.29688; “Pump P-101” stays at 0.",
    title: "F05 · Alignment",
    values: createTextStepValues({}),
  },
  {
    explanation: "Right to left, Alignment Left; then Center and Right.",
    look: "Left: each line starts on the frame's left edge; tspan x 175.90625 and 123.60938.",
    title: "F05 · Right to left: left is the left side (decision)",
    values: createTextStepValues({ direction: "rtl" }),
  },
  {
    explanation: "Nothing to type: Hebrew, right to left.",
    look: "The letters run from the right; Inter has none, so each is a 21-wide missing-glyph box.",
    title: "F05 · Hebrew",
    values: createTextStepValues({ content: "שלום\nעולם טוב", direction: "rtl" }),
  },
  {
    explanation: "X 100, Y 50.",
    look: "Text and frame move together.",
    title: "F05 · Position",
    values: createTextStepValues({}),
  },
  {
    explanation: "Angle 90; Pipeline.",
    look: "A quarter turn clockwise about the frame's middle: rotate(90 187.95313 88.71875).",
    title: "F05 · Turned",
    values: createTextStepValues({ x: 100, y: 50 }),
  },
  {
    explanation: "Angle 370, Enter; −90, Enter; 360, Enter; then +90°, −90°, −90°.",
    look: "10, 270, 0 (no transform); then 90, 0, 270.",
    title: "F05 · Angle kept from 0 to 359",
    values: createTextStepValues({}),
  },
  {
    explanation: "Nothing to type: the text turned by 30.",
    look: "The frame turns with the letters, thin and blue as a turned shape's box, an upright dashed box around it (US-050).",
    title: "F05 · The dashed frame",
    values: createTextStepValues({ angle: 30 }),
  },
  {
    explanation: "Pipeline: SVG output.",
    look: "rotate(90 …) on the <g> holding the <text> and its 4 stroke paths: strokes turn too.",
    title: "F05 · Turned as a group (decision)",
    values: createTextStepValues({ angle: 90, isStruckThrough: true, isUnderlined: true }),
  },
  {
    explanation: "Size 0; Size 16. Opacity 101; 100. Color red; #1e293b.",
    look: "Each refused: drawing gone, message, fields in red; back when valid.",
    title: "F05 · Refused values",
    values: createTextStepValues({}),
  },
  {
    explanation: "The Rectangle tool, then a click on the drawing.",
    look: "The text stays as it was, at 100, 50, turned by 90; a rectangle 100 × 100 added on top, straight, centred on the click; the tool back to the selector.",
    title: "F05 · A new shape leaves the text as it was (US-063)",
    values: createTextStepValues({ angle: 90, x: 100, y: 50 }),
  },
];

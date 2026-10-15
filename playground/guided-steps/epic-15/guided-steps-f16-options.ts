import { createColorStepValues } from "./create-color-step-values";

import type { GuidedStep } from "../guided-step";

/** "Pump P-101" in the text's opening color, #1e293b. */
const OPENING = createColorStepValues({});

/** "Pump P-101" in orange, HSB 30°, 100 %, 100 %. */
const ORANGE = createColorStepValues({ color: "#ff8000" });

/**
 * The F16 options to try (VAL-016), one per step: the hexadecimal forms typed or pasted, a value
 * refused, the opacity, the swatch, the picker opened and closed, its sliders by mouse and keys,
 * its plane in HSB and HSL, its values typed, a grey and the background's color.
 */
export const GUIDED_STEPS_F16_OPTIONS: readonly GuidedStep[] = [
  {
    explanation: "Color: FF8000, then Tab.",
    look: "#ff8000 in the field: # added, lower case; the text and the swatch orange.",
    title: "F16 · Six digits",
    values: OPENING,
  },
  {
    explanation: "Color: “ #FF8000 ”, a space on each side, then Enter.",
    look: "#ff8000: the spaces go, the letters in lower case.",
    title: "F16 · With # and spaces",
    values: OPENING,
  },
  {
    explanation: "Color: f80, then Tab.",
    look: "#ff8800: each digit doubled.",
    title: "F16 · Three digits",
    values: OPENING,
  },
  {
    explanation: "Color: #F80C, then Tab.",
    look: "#ff8800, and Opacity 80: C is CC, 204 of 255.",
    title: "F16 · Four digits",
    values: OPENING,
  },
  {
    explanation: "Color: #0000ffcc, then Tab.",
    look: "#0000ff, and Opacity 80: the text blue, a little transparent.",
    title: "F16 · Eight digits",
    values: OPENING,
  },
  {
    explanation: "Copy #22C55E from this line, paste it into Color, then Tab.",
    look: "#22c55e: the text green.",
    title: "F16 · Pasted",
    values: OPENING,
  },
  {
    explanation: "Color: red, Tab; then #ff800, Tab; then FF8000, Tab.",
    look: "#1e293b back twice, a red line under it saying why; then #ff8000, the line gone.",
    title: "F16 · Refused, the color back",
    values: OPENING,
  },
  {
    explanation: "Opacity 40.",
    look: "The text lighter; Model column: “opacity”: 40, the color still #ff8000.",
    title: "F16 · Opacity",
    values: ORANGE,
  },
  {
    explanation: "Click the swatch left of Color.",
    look: "The picker opens under the color's fields: plane, hue bar, opacity bar, HSB 30, 100, 100.",
    title: "F16 · The swatch opens the picker",
    values: ORANGE,
  },
  {
    explanation: "Click the swatch twice; then once and Escape; then once and click the drawing.",
    look: "Closed each time; after Escape, the swatch has the focus.",
    title: "F16 · The picker closed three ways",
    values: ORANGE,
  },
  {
    explanation: "Open the picker; drag the hue bar's thumb to the right, then past the bar's end.",
    look: "H rises, the text and the plane follow; past the end it stays at the end: H 0, #ff0000.",
    title: "F16 · Hue bar by mouse",
    values: ORANGE,
  },
  {
    explanation: "Open the picker; Tab four times to the hue bar; Right, Page Up, Home, End.",
    look: "H 210, then 211, 221, 0, 359; the text follows.",
    title: "F16 · Hue bar by keys",
    values: createColorStepValues({ color: "#336699" }),
  },
  {
    explanation:
      "Open the picker; click the opacity bar's middle; then on it Left, Page Down, End.",
    look: "Opacity near 50, then 1 less, 10 less, 100: the field and the text follow.",
    title: "F16 · Opacity bar",
    values: ORANGE,
  },
  {
    explanation:
      "Open the picker, Model HSB; click the plane's top left, its bottom, its right edge's middle.",
    look: "#ffffff (S 0, B 100); #000000 (B 0); #804000 (S 100, B 50).",
    title: "F16 · Plane in HSB",
    values: ORANGE,
  },
  {
    explanation: "Open the picker, Model HSL; click the plane's top, then its right edge's middle.",
    look: "L instead of B; #ffffff at the top (L 100); #ff8000 again at the middle (S 100, L 50).",
    title: "F16 · Plane in HSL",
    values: ORANGE,
  },
  {
    explanation: "Open the picker, Model HSB; S 50, Tab; B 50, Tab.",
    look: "#ffc080, then #806040, H still 30; the mark and the thumbs move to the values.",
    title: "F16 · Values typed",
    values: ORANGE,
  },
  {
    explanation: "Open the picker; H 200, Tab; then click the plane's top right.",
    look: "A grey opens at H 0; H 200 kept: #00aaff, a sky blue.",
    title: "F16 · A grey keeps its hue",
    values: createColorStepValues({ color: "#808080" }),
  },
  {
    explanation: "Click the Background color's swatch; H 120, Tab.",
    look: "The picker under the background's fields at H 60; the band #00ff00, the letters unchanged.",
    title: "F16 · The background's color",
    values: createColorStepValues({ hasBackground: true }),
  },
];

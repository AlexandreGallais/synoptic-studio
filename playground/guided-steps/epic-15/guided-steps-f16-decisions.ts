import { createColorStepValues } from "./create-color-step-values";
import { createTextBoxStepValues } from "./create-text-box-step-values";

import type { GuidedStep } from "../guided-step";

/** "Pump P-101" in the text's opening color, #1e293b. */
const OPENING = createColorStepValues({});

/** "Pump P-101" in orange, HSB 30°, 100 %, 100 %. */
const ORANGE = createColorStepValues({ color: "#ff8000" });

/** "Pump P-101" in a steel blue, HSB 210°, 67 %, 60 %. */
const STEEL_BLUE = createColorStepValues({ color: "#336699" });

/**
 * The F16 decisions to confirm (VAL-016), one per row of the story's table with its example, then
 * back to F14 and F05 and to the rectangle (F16.AC4).
 */
export const GUIDED_STEPS_F16_DECISIONS: readonly GuidedStep[] = [
  {
    explanation: "Open the picker, Model HSL.",
    look: "HSL 30, 100, 50; the Model column holds “color”: “#ff8000”, “opacity”: 100, no HSL.",
    title: "F16 · Hexadecimal stored (decision 1)",
    values: ORANGE,
  },
  {
    explanation: "Nothing to type: Next wrote #FF8000. Then Color “ #ff8000 ”, spaces around, Tab.",
    look: "Drawn #ff8000 at once, read in lower case; after Tab the field shows #ff8000.",
    title: "F16 · Forms typed (decision 2)",
    values: createColorStepValues({ color: "#FF8000" }),
  },
  {
    explanation: "Color f80, Tab; then #0000ffcc, Tab; then #ff8000ec, Tab.",
    look: "#ff8800; #0000ff and Opacity 80; #ff8000 and Opacity 93 (ec is 236 of 255, 92.5 %).",
    title: "F16 · Three digits, and an alpha (decision 3)",
    values: OPENING,
  },
  {
    explanation: "Color red, Tab; #ff800, Tab. Then copy Color and paste it into Background color.",
    look: "Both refused, #ff8000 back; the copy is #ff8000, six digits, Opacity left apart.",
    title: "F16 · Anything else refused; a copy (decision 4)",
    values: createColorStepValues({ color: "#ff8000", hasBackground: true }),
  },
  {
    explanation: "Swatch; swatch again; swatch and Escape; swatch and a click on the drawing.",
    look: "Open; closed; open then closed; open then closed.",
    title: "F16 · Opened and closed (decision 5)",
    values: ORANGE,
  },
  {
    explanation: "Reload the page (it opens on this step); open the picker; then Model HSL.",
    look: "Model HSB first, its third value B; HSL one choice away, its third value L.",
    title: "F16 · HSB first (decision 6)",
    values: ORANGE,
  },
  {
    explanation:
      "Open the picker, Model HSB; H 210, Tab; click the plane's right edge in its middle.",
    look: "#004080: S grows to the right, 100 at the edge; B grows upwards, 50 halfway.",
    title: "F16 · The plane's axes (decision 7)",
    values: ORANGE,
  },
  {
    explanation: "Open the picker, Model HSB; S 67 as shown, Tab; then B 61, Tab.",
    look: "210, 67, 60, no decimal; S 67 leaves #336699; B 61 gives #34689c, not #33679c; S still 67.",
    title: "F16 · Whole numbers, the others exact (decision 8)",
    values: STEEL_BLUE,
  },
  {
    explanation: "Open the picker, Model HSL; H 30, S 100, L 50, each then Tab.",
    look: "#ff8000: the green 127.5 rounded up to 128, 80 in hexadecimal.",
    title: "F16 · Rounded, a tie up (decision 9)",
    values: OPENING,
  },
  {
    explanation: "Open the picker, Model HSB; H 200, Tab; drag the plane's mark to the right.",
    look: "The grey opens at H 0; H 200 kept as the saturation grows: a blue, #406b80, not a red.",
    title: "F16 · A grey's hue (decision 10)",
    values: createColorStepValues({ color: "#808080" }),
  },
  {
    explanation:
      "Open the picker, Model HSB; Tab 4 times to the hue bar; Right, Page Up; Shift+Tab, Right, Up.",
    look: "H 211, 221; on the plane S 68, then B 61.",
    title: "F16 · Keys (decision 11)",
    values: STEEL_BLUE,
  },
  {
    explanation: "Open the picker; Tab 4 times to the hue bar; Right; End. Then H 360, Tab.",
    look: "H 30, then 31, 359; H 360 shown 0.",
    title: "F16 · Whole degrees of hue (decision 12)",
    values: ORANGE,
  },
  {
    explanation: "Click the Background color's swatch; then #22c55e in Background color, Tab.",
    look: "The same swatch and picker under the background's fields; the band green.",
    title: "F16 · The background's color too (decision 13)",
    values: createColorStepValues({ hasBackground: true }),
  },
  {
    explanation: "Color: red, then Tab.",
    look: "#ff8800 again, the color it had when it got the focus; a line under it says why.",
    title: "F16 · Refused, the color back (decision 14)",
    values: createColorStepValues({ color: "#ff8800" }),
  },
  {
    explanation: "Open the picker; H empty, Tab; H 400, Tab; H −10, Tab; click the opacity bar.",
    look: "Empty: 30 shown again; H 400 gives 0, −10 gives 0; the opacity a whole percent.",
    title: "F16 · Beyond the ends (decision 15)",
    values: ORANGE,
  },
  {
    explanation: "Open the picker; Opacity 50 in its field; press Escape.",
    look: "The picker under the fields, the opacity bar at 50; Escape closes it, the swatch focused.",
    title: "F16 · Under its fields; Escape (decision 16)",
    values: ORANGE,
  },
  {
    explanation: "Open the picker, Model HSB; then Model HSL.",
    look: "HSB 210, 2, 50; HSL 210, 1, 50: the color's own hue in both, #7e7f80 kept.",
    title: "F16 · Another model, its own hue (decision 17)",
    values: createColorStepValues({ color: "#7e7f80" }),
  },
  {
    explanation:
      "Open the picker; a screen reader on (VoiceOver: Cmd+F5); Tab 4 times to the hue bar.",
    look: "“Hue, 30, slider”; the focus ring around the bar, the thumb only drawn.",
    title: "F16 · Read by a screen reader (decision 18)",
    values: ORANGE,
  },
  {
    explanation: "Open the picker, Model HSB; then Color #030330, Tab.",
    look: "H 196 (196.25 exactly); then S 94 (93.75 exactly): no tenth shown.",
    title: "F16 · No tenth shown (decision 19)",
    values: createColorStepValues({ color: "#002330" }),
  },
  {
    explanation: "Open the picker, Model HSB; Tab 3 times to the plane; Page Up; End.",
    look: "S 77, then 100; B stays 60.",
    title: "F16 · Page keys on the plane (decision 20)",
    values: STEEL_BLUE,
  },
  {
    explanation: "Strike-through color #ff0000, Tab; then Color #22c55e, Tab.",
    look: "Both strokes #1e293b at first; the strike-through red, its box unticked; then the letters and the underline green, the strike-through still red.",
    title: "F16 · The strokes follow the text until chosen (decision 21)",
    values: createColorStepValues({ isStruckThrough: true, isUnderlined: true }),
  },
  {
    explanation: "Nothing to type.",
    look: "As in F14: “Hello World” in its box, the yellow band, the letters #1e293b (F16.AC4).",
    title: "F16 · Back to F14 and F05",
    values: createTextBoxStepValues({ hasBackground: true, isInBox: true }),
  },
  {
    explanation: "Rectangle.",
    look: "The sharp rectangle 120 × 80, as before; the text's fields and the picker go.",
    title: "F16 · Back to the rectangle",
    values: ORANGE,
  },
];

import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E15's checklist on colors, fonts and F17's corrections (US-051): F16's options,
 * then F15's, then what F17 fixed.
 */
export const EPIC_15_COLOR_AND_FONT_LINES: readonly ChecklistLine[] = [
  {
    feature: "F16",
    todo: "Type ff8000, then  #FF8000 , f80, 00fc and #0000ffcc in Color",
    see: "#ff8000 kept, # added, lower case; #ff8800; four or eight digits also set the opacity",
  },
  {
    feature: "F16",
    todo: "Type red, then #ff800",
    see: "refused: the color before comes back, a line under it says why",
  },
  {
    feature: "F16",
    todo: "Click the swatch beside Color, then again; open it and press Escape",
    see: "the picker under the fields: plane, hue bar, opacity bar, HSB values; closed each time",
  },
  {
    feature: "F16",
    todo: "Drag the hue bar, use the arrows, Page Up, Home and End on it; drag the opacity bar",
    see: "H in whole degrees, the text and the plane following; the opacity a whole percent",
  },
  {
    feature: "F16",
    todo: "Drag on the plane in HSB, then choose HSL",
    see: "saturation across, brightness up (lightness in HSL); white at the top left in HSB",
  },
  {
    feature: "F16",
    todo: "Type #336699, then B 61 in the picker",
    see: "HSB 210, 67, 60 as whole numbers; B 61 gives #34689c, S still 67",
  },
  {
    feature: "F16",
    todo: "#808080, open the picker, set H 200, then drag towards more saturation",
    see: "the grey opens at H 0; H 200 kept: a blue, not a red",
  },
  {
    feature: "F16",
    todo: "Open the background's, the underline's and the strike-through's swatches",
    see: "each its own picker; a stroke follows the text's color until its own is chosen",
  },
  {
    feature: "F15",
    todo: "Open the Font list",
    see: "Inter, then the fonts imported, then the installed ones; no name can be typed",
  },
  {
    feature: "F15",
    todo: "Import a font: Arial (/System/Library/Fonts/Supplemental on macOS), then a file that is not a font",
    see: "“Arial 400 imported.”, Font Arial at once, the text narrower; the other file refused, saying why",
  },
  {
    feature: "F15",
    todo: "Open the Weight list with Arial",
    see: "only the weights the family has: 400 Regular and 700 Bold",
  },
  {
    feature: "F15",
    todo: "Installed fonts in Chrome or Edge, then choose Georgia",
    see: "the computer's families listed; “Georgia: 4 faces read.” once chosen; elsewhere, a line saying to import the file",
  },
  {
    feature: "F15",
    todo: "Choose the installed Helvetica, held in a .ttc collection",
    see: "“Helvetica: 6 faces read.”, none refused; a face the browser cannot give said in one line (US-046)",
  },
  {
    feature: "F15",
    todo: "Import a font: Helvetica.ttc (/System/Library/Fonts on macOS)",
    see: "“Helvetica 400, Helvetica 700, … imported.”, its faces in the Weight list (US-046)",
  },
  {
    feature: "F15",
    todo: "IBM Plex Sans installed, weight 400, then 450, then 500",
    see: "three different weights, 450 a little heavier than 400, the frame hugging the letters at each (US-047)",
  },
  {
    feature: "F15",
    todo: "Embed fonts on, a text in Arial",
    see: "the drawing on screen unchanged, no <style> in SVG output; the faces left out listed, “Inter 400: not embedded, its file not held.”",
  },
  {
    feature: "F15",
    todo: "Export SVG, then open drawing.svg as text",
    see: "one @font-face per face used, no dashed frame, no <switch> nor <foreignObject> (US-044, US-045)",
  },
  {
    feature: "F15",
    todo: "Reload the page",
    see: "Inter alone in the list: a font imported is kept for the session only",
  },
  {
    feature: "F17",
    todo: "A text in a box: hold the Up arrow on Size, then on Opacity and the box's Height",
    see: "the drawing follows smoothly, no lag, stopping when the key is released (US-048)",
  },
  {
    feature: "F17",
    todo: "A rectangle 60 × 40 turned by 30, then a text in a box 60 × 40 turned by 30",
    see: "both: the turned frame thin and blue, the straight box dashed around it (US-050)",
  },
  {
    feature: "F17",
    todo: "Draw the rectangle, the polygon and the round shape",
    see: "each drawn as before E15",
  },
];

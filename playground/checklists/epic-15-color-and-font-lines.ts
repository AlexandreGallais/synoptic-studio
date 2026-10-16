import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E15's checklist on colors, fonts and F17's corrections (US-051): F16's options,
 * then F15's, then what F17 fixed, with VAL-018's readings to confirm, questions and warning.
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
    todo: "Fit Box, then look at the box's fields and the Drawing's options",
    see: "Width, Height, Max lines, Ellipsis, Vertical alignment; Embed fonts alone: no option for a word too long nor for how lines fill (US-042, US-043, US-044)",
  },
  {
    feature: "F17",
    todo: "Reading to confirm (US-046): Installed fonts, a family one face of which Chrome cannot give",
    see: "its line says “<full name>: <Chrome's message> Import its file with Import a font”, the other faces kept; from a .ttc, only the face of its PostScript name",
  },
  {
    feature: "F17",
    todo: "Reading to confirm (US-049): “Hello World ”, Arial 16, box 50, Alignment Right, Background on",
    see: "“World” ends at the box's edge, x 8.219, its band ending with it: the last space hung; kept, as CSS pre-wrap does, it would end at 3.773",
  },
  {
    feature: "F17",
    todo: "Reading to confirm (US-047): Import a font file the browser refuses to load",
    see: "one line, “Broken 450 italic could not be loaded by the browser (…): measured with it, drawn with another face.”",
  },
  {
    feature: "F17",
    todo: "Question (AUD-019): Installed fonts, Helvetica Neue, “Pump P-101” Size 32, weight 400, then 700",
    see: "at 700 the Condensed Bold, 155.08 wide, narrower than at 400, 180.27: say which face 700 should draw",
  },
  {
    feature: "F17",
    todo: "Question (AUD-019): in Safari, Import a font whose family has a digit, Source Sans 3",
    see: "drawn in Source Sans 3: the name kept unquoted; refused or drawn in another face: names to quote",
  },
  {
    feature: "F17",
    todo: "Reading to confirm (US-052): Installed fonts, choose Apple Color Emoji; then Import a font, Apple Color Emoji.ttc",
    see: "no crash: its faces read, or one line naming the face or the file and the browser's reason",
  },
  {
    feature: "F17",
    todo: "Reading to confirm (US-053, US-056): Installed fonts, allow, wait; open the font list, then choose Helvetica",
    see: "a line counting the families read, then those left out (ABC Oracle Book gone from the list); no Apple Color Emoji, Al Nile nor Apple Braille: the line counts them under “no Latin letters”, and a font whose licence forbids embedding under “not embeddable”; Helvetica 300, 400 and 700 only, “Helvetica: 6 faces read.”",
  },
  {
    feature: "F17",
    todo: "Reading to confirm (US-056): Inter not installed, open the page, Text tool; Installed fonts, allow; then import Inter's Regular and Bold files",
    see: "Installed fonts first and in blue, an empty font list, no text; the first family read chosen and the text drawn in it; once imported, Inter listed with 400 and 700 only",
  },
  {
    feature: "F17",
    todo: "Draw the rectangle, the polygon and the round shape",
    see: "each drawn as before E15",
  },
];

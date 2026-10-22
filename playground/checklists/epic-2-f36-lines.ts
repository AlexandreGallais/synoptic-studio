import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E02's checklist for F36, Draw and pick shapes.
 *
 * Prepared by `val-prepare` from its acceptance criteria (EN-056), then written as what to
 * try and what to see, at most 12: a drag in each direction, with Shift, with the Text tool,
 * released on its press and straight down; the hover with the Selector, on a text, on a shape
 * of width 0; a click picking a shape, a text with its style, none; and the decisions of
 * US-064, EN-143, US-065 and US-067 to confirm, with the question US-067 left.
 */
export const EPIC_2_F36_LINES: readonly ChecklistLine[] = [
  {
    feature: "F36",
    todo: "Reset; Rectangle, press the drawing right of the rectangle, drag down and to the right, release; then Rectangle, a drag up and to the left",
    see: "the rectangle follows the pointer from the press, in any direction, and stays at the release, X, Y, Width and Height whole numbers, the Selector pressed again; the second drawn on top, its X and Y the point released; Decision to confirm (US-064): the press and the pointer each rounded to the nearest integer, a half up, as a click, the box from the smaller to the larger of each coordinate",
  },
  {
    feature: "F36",
    todo: "Round, a drag with Shift held; then Polygon, a drag, Shift pressed then released before the release",
    see: "a circle, its side the larger of the two distances dragged, laid from the press toward the pointer; the polygon's box square while Shift is held, not once it is released; Decision to confirm (US-064): with Shift the side is the larger distance, laid from the press toward the pointer",
  },
  {
    feature: "F36",
    todo: "Text, a drag of a wide and low box with Shift held, then type Pump",
    see: "the text in a box of the size dragged, not square, Fit Box, Text selected in Content and replaced by Pump; Decision to confirm (US-064): a text is never squared",
  },
  {
    feature: "F36",
    todo: "Rectangle, press and release on one point; then Polygon, press, drag away and back to the press, release",
    see: "each a shape 100 × 100 centred on the point pressed, as a click places it; Decision to confirm (US-064): a release on the press's point is a click, even after the pointer went away and came back",
  },
  {
    feature: "F36",
    todo: "Rectangle, a drag straight down (Width 0, else again); click the empty drawing; the pointer 2 pixels beside the drag's line, then 10",
    see: "nothing drawn, the panel Rectangle, Width 0, the height dragged; outlined with the pointer 2 pixels away, not 10; Decision to confirm (US-064): a side of 0 kept as drawn; Decision to confirm (EN-143): a shape of width 0 under the pointer within 4 pixels of the screen",
  },
  {
    feature: "F36",
    todo: "Reset; Round, a click on the rectangle's right end; click the empty drawing, then the pointer over the rectangle, its rounded corner, the circle, its box's corner; then Rectangle, the same",
    see: "a band 2 pixels wide along the edge of the shape under the pointer, the circle where both overlap, gone once it leaves; nothing in the rounded corner nor outside the circle; nothing with Rectangle pressed; Decision to confirm (US-065): the band along the contour as drawn, only with the Selector",
  },
  {
    feature: "F36",
    todo: "Text, a click, type Pump Valve; click the empty drawing; the pointer between the two words, then below the text",
    see: "the text outlined along its block between the words, nothing below it; Decision to confirm (EN-143): a text is under the pointer anywhere in its block, or its box when in one, not only on its letters",
  },
  {
    feature: "F36",
    todo: "Reset; Round, a click on the rectangle's right end; click the rectangle's left end, Radius 30; then Polygon, a click",
    see: "the rectangle outlined as selected, the panel Rectangle, X 0, Y 0, Width 120, Height 80, Radius 10; Radius 30 rounds it under the circle, which stays on top; the new polygon above all",
  },
  {
    feature: "F36",
    todo: "Text, a click, Weight 700 Bold, Size 40, type Pump; Text, a click below it, Weight 400 Regular, Size 24, type Valve; click Pump",
    see: "the panel Text, Pump in Content, its font chosen in the list, Weight 700 Bold, Size 40; Pump unchanged and in its place",
  },
  {
    feature: "F36",
    todo: "Reset; click the rectangle, then Polygon; a drag from the empty drawing up and to the left, released on the rectangle; click the new polygon",
    see: "the rectangle outlined while Polygon is pressed; at the release the new polygon selected, not the rectangle under the pointer; the click changes nothing; Decision to confirm (US-067): the selection shown whatever tool is armed, as Figma does; a click picks only when pressed with the Selector, again on the shape selected",
  },
  {
    feature: "F36",
    todo: "Click the empty drawing; then Rectangle, a click",
    see: "every shape drawn, none outlined; the panel No shape selected, its fields hidden, its status how to select or add one, Export SVG not shown; then the new rectangle edited; Decision to confirm (US-067): none selected hides the fields; To settle (US-067): Export SVG cannot be reached with none selected",
  },
  {
    feature: "F36",
    todo: "Read the question: a text picked whose family is no longer in the font list, its files removed during the session",
    see: "Question (US-067): loading it measures it with the family the list falls back to, which changes a kept text; (1) load it anyway, its family changed; (2) select it without loading its family until a field is typed; (3) show it selected with a message, its fields read-only",
  },
];

import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E02's checklist for F35, Several shapes on one drawing.
 *
 * Prepared by `val-prepare` from its acceptance criteria (EN-056), then written as what to
 * try and what to see, at most 12: each tool placing a shape on a click, the starting values
 * SP-026 set, the export, and the decisions of SP-026, US-063 and AUD-037 to confirm, the last
 * two by a test run in a terminal, since no font missing is easy to bring about in a browser.
 */
export const EPIC_2_F35_LINES: readonly ChecklistLine[] = [
  {
    feature: "F35",
    todo: "Reset, then click Rectangle",
    see: "the rectangle 120 × 80 the page opens with, alone; Rectangle pressed, the drawing and the panel unchanged; Decision to confirm (US-063): a tool's button only arms the tool, as Figma does",
  },
  {
    feature: "F35",
    todo: "Click the drawing right of the rectangle, then click the drawing again",
    see: "a rectangle 100 × 100 centred on the click, above the first, which does not change; the panel Rectangle, Width 100, Height 100, Radius 10; Selector pressed again; the second click adds nothing; Decision to confirm (US-063): the Selector places nothing until F36",
  },
  {
    feature: "F35",
    todo: "Polygon, a click on the drawing; then Round, a click elsewhere",
    see: "a hexagon of radius 10 in a box 100 × 100 centred on the click, the page's 6 corners and its anchor at the center; then a circle 100 × 100 centred on its click, each above the shapes before",
  },
  {
    feature: "F35",
    todo: "Polygon, a click; Corners 3, Radius 4, Anchor Bottom center; Rectangle, a click, Radius 0; then Polygon and Rectangle, a click each",
    see: "a triangle of radius 4 on its box's bottom, straight, 100 × 100 on its click; then a rectangle of radius 0; Decision to confirm (SP-026): a new shape takes the settings of the last of its kind, the page's before any",
  },
  {
    feature: "F35",
    todo: "Text, a click on the drawing, then type Pump",
    see: "Text centred on the click, the Text panel shown, Text selected in Content; Pump replaces it; Decision to confirm (SP-026): a new text's content Text, selected, its options the last text's",
  },
  {
    feature: "F35",
    todo: "Zoom the browser to 110 % (Cmd +), Rectangle, a click on the drawing; then Cmd 0",
    see: "X and Y of the new rectangle whole numbers; Decision to confirm (US-063): the corner rounded to the nearest integer, a half up, as CSS rounds: (250.5, 140.4) gives (201, 90)",
  },
  {
    feature: "F35",
    todo: "Empty Width, then Rectangle and a click on the drawing",
    see: "the empty width refused, the page saying why; the new rectangle drawn, the refused one gone; Decision to confirm (US-063): a shape refused is not kept when another is placed",
  },
  {
    feature: "F35",
    todo: "Polygon, a click, Corners 3; Points, Convert to free shape, Shape; then Polygon, a click",
    see: "the triangle becomes a Free shape; the new polygon a triangle; Decision to confirm (US-063): a rectangle or a polygon made a free shape keeps its settings for the next of its kind",
  },
  {
    feature: "F35",
    todo: "Text, a click, Size 20, Weight 700 Bold; Reset; then Text, a click, and Polygon, a click",
    see: "the page's rectangle alone; the text Inter 32 Regular, the polygon a hexagon of radius 10; Decision to confirm (US-063, AUD-037): Reset and each guided step forget the shapes placed and the settings kept, the Text tool's too",
  },
  {
    feature: "F35",
    todo: "Place a rectangle, a polygon and a text over each other, then Export SVG and open the file",
    see: "every shape drawn, in the order drawn, the last on top, none changed by the others; no dashed frame nor handle",
  },
  {
    feature: "F35",
    todo: 'Terminal: npx vitest run playground/placing/place-shape.test.ts -t "metrics"',
    see: "3 passed, 20 skipped; Decision to confirm (US-063): a text placed while no font measures it has its top-left on the click; Decision to confirm (AUD-037): a kept text whose font is no longer read is drawn and exported as last drawn",
  },
];

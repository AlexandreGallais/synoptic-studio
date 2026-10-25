import type { GuidedStep } from "../guided-step";

/** The hexagon of the F08 steps: 100 × 100, sharp. */
const HEXAGON = {
  anchor: "mid mid",
  corners: 6,
  height: 100,
  radius: 0,
  shape: "polygon",
  width: 100,
} as const;

/** The rectangle of the F08 steps: 120 × 80, rounding 10. */
const RECTANGLE = {
  anchor: "mid mid",
  corners: 6,
  height: 80,
  radius: 10,
  shape: "rectangle",
  width: 120,
} as const;

/** The steps of the US-014, US-015 and US-016 cards, one at a time. */
export const GUIDED_STEPS_F08: readonly GuidedStep[] = [
  {
    explanation:
      "F08 starts here: a hexagon. Click Points, then point 1 (red), then “Convert to free shape”. A free shape is one whose points move one by one.",
    look: "Nothing moves; the shape selector reads “Free shape”, width 100, height 86.60254; Corners and the anchor grid go away.",
    title: "F08 · Convert a polygon",
    values: HEXAGON,
  },
  {
    explanation:
      "Convert again as in the step before, select point 1, then type X 80 and press Enter.",
    look: "The bottom-right corner moves right to 80; its Y keeps 93.30127; the other points stay.",
    title: "F08 · Move a point by its x",
    values: HEXAGON,
  },
  {
    explanation: "Still on the converted hexagon with point 1 moved, type width 50.",
    look: "The whole shape is stretched to 50 wide, its left side fixed; every point follows.",
    title: "F08 · Stretch after a move",
    values: HEXAGON,
  },
  {
    explanation:
      "The rectangle is a free shape already: click Points, then point 3, type X 150 and press Enter.",
    look: "The bottom-right corner goes to 150; the selector reads “Free shape”, width 150. Then type X 150.5: nothing moves.",
    title: "F08 · Move a point of the rectangle",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 1, then Ctrl + click point 2 (Cmd on a Mac), then “Add a point in the middle”; then type Y -20 and press Enter.",
    look: "A sharp point 2 at x 60, y 0, selected alone; then a peak 20 above the top side.",
    title: "F08 · Add a point in the middle",
    values: RECTANGLE,
  },
  {
    explanation: "Click Points, then point 2 (top right), then “Delete points” or the delete key.",
    look: "A triangle: the top-left corner joined to the bottom-right one; three points listed.",
    title: "F08 · Delete a point",
    values: RECTANGLE,
  },
  {
    explanation: "Click Points, point 1, then Ctrl + click point 2.",
    look: "“Delete points” is greyed, and a line says a shape keeps at least 3 points.",
    title: "F08 · At least 3 points",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, then point 1: a regular polygon keeps its points until it is converted.",
    look: "X and Y greyed, adding and deleting greyed, a line saying to convert the polygon first; the points have a Radius field of their own, the shape's Radius field still rounding every corner.",
    title: "F08 · A polygon is converted first",
    values: HEXAGON,
  },
];

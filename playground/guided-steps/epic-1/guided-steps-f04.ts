import type { GuidedStep } from "../guided-step";

/** The rectangle of the F04 steps: 120 × 80, sharp, so that the numbers are round. */
const RECTANGLE = {
  anchor: "mid mid",
  corners: 6,
  height: 80,
  radius: 0,
  shape: "rectangle",
  width: 120,
} as const;

/** The triangle of the F04 steps: 100 × 100, its base on the bottom of its box. */
const TRIANGLE = {
  anchor: "mid max",
  corners: 3,
  height: 100,
  radius: 0,
  shape: "polygon",
  width: 100,
} as const;

/** The square of the F04 steps, a polygon of 4 corners in 100 × 100. */
const SQUARE = { ...TRIANGLE, anchor: "mid mid", corners: 4 } as const;

/** The steps of the US-018 and US-019 cards, one at a time; each starts straight. */
export const GUIDED_STEPS_F04: readonly GuidedStep[] = [
  {
    explanation:
      "F04 starts here: a 120 × 80 rectangle, straight. Type 90 in Angle. The angle turns the shape clockwise about the center of its box.",
    look: "It stands up about its center, and may go above the canvas; a thin blue frame (its turned box) and a dashed box (its straight box); “Straight box: x 20, y -20, width 80, height 120”; Width and Height still 120 and 80.",
    title: "F04 · Turn by 90°",
    values: RECTANGLE,
  },
  {
    explanation: "Type 10 in Radius, then 45 in Angle.",
    look: "The corners of the blue frame stick out of the dashed box by about 4: the rounded corners take less room; “… width 133.13708, height 133.13708”.",
    title: "F04 · Tilt by 45°",
    values: RECTANGLE,
  },
  {
    explanation: "Type 370 in Angle, then Enter; then type -90, then Enter.",
    look: "The field shows 10, then 270: the angle is kept from 0 to 359.",
    title: "F04 · The angle from 0 to 359",
    values: RECTANGLE,
  },
  {
    explanation: "Click +90° four times, then −90° once.",
    look: "90, 180, 270, 0 — straight again, exactly as at the start — then 270.",
    title: "F04 · Quarter turns",
    values: RECTANGLE,
  },
  {
    explanation: "Type 90 in Angle, then 300 in X and 100 in Y.",
    look: "The shape moves so that its dashed box starts at (300, 100): “Straight box: x 300, y 100, width 80, height 120”. Turn it again: it turns on itself.",
    title: "F04 · Move by the straight box",
    values: RECTANGLE,
  },
  {
    explanation: "A triangle whose base touches the bottom of its grey box. Type 90 in Angle.",
    look: "The base is on the left; the grey box turned with it.",
    title: "F04 · The box turns with its content",
    values: TRIANGLE,
  },
  {
    explanation: "A square. Type 45 in Angle; then 6 in Corners and 30 in Angle.",
    look: "A diamond (ISA); then a hexagon standing on a vertex (IEC).",
    title: "F04 · Diamond and standing hexagon",
    values: SQUARE,
  },
  {
    explanation: "Type 90 in Angle, then 200 in Width; then 0 in Angle.",
    look: "Point 1, drawn at the top right (100, −20), does not move: “… width 80, height 200”; then a 200 × 80 rectangle, straight.",
    title: "F04 · Width of a turned shape",
    values: RECTANGLE,
  },
  {
    explanation:
      "Type 90 in Angle, click Points, then point 1 in the list; type 110 in X, then Enter.",
    look: "X first shows 100, the point as drawn; then only point 1 moves, 10 to the right; the other corners stay.",
    title: "F04 · Move a point of a turned shape",
    values: RECTANGLE,
  },
  {
    explanation: "Type 90 in Angle, click Points, then point 1, then “Delete points”.",
    look: "A triangle of the three other points, which do not move.",
    title: "F04 · Delete a point of a turned shape",
    values: RECTANGLE,
  },
  {
    explanation: "Type 30 in Angle, click Points, then “Convert to free shape”.",
    look: "Nothing moves; the selector reads “Free shape”; Angle still 30.",
    title: "F04 · Convert a turned polygon",
    values: TRIANGLE,
  },
];

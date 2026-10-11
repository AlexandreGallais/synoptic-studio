import type { GuidedStep } from "./guided-step";

/** The ten steps of US-007's test card and the one of US-008 (F02, VAL-002), one at a time. */
export const GUIDED_STEPS_F02: readonly GuidedStep[] = [
  {
    explanation:
      "F02 starts here. A 100 × 100 box, radius 0, and the shape switched to Polygon: a regular hexagon (6 equal sides) with a flat base, as large as the box allows without being stretched.",
    look: "The hexagon touches the left and right of the box, not its top and bottom (it is 86.6 high). The Corners field appears with 6; width, height and radius kept.",
    title: "F02 · A hexagon in its box",
    values: { anchor: "mid mid", corners: 6, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation: "3 corners: a triangle, pointing up, its base flat.",
    look: "100 wide, 86.6 high, centered vertically in the box.",
    title: "F02 · A triangle",
    values: { anchor: "mid mid", corners: 3, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation: "4 corners: a square with a flat base — not a diamond.",
    look: "A 100 × 100 square.",
    title: "F02 · A square, not a diamond",
    values: { anchor: "mid mid", corners: 4, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation: "Height 50: the square shrinks to stay a square, and is centered in the width.",
    look: "A 50 × 50 square, from 25 to 75 across.",
    title: "F02 · Kept regular",
    values: { anchor: "mid mid", corners: 4, height: 50, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation:
      "Back to the hexagon with a radius of 1000, far too big: every corner is rounded as much as it can be, and the hexagon becomes the circle inside it.",
    look: "A circle of diameter 86.6, and “Effective radius: 43.30127 (requested 1000, reduced to fit)”.",
    title: "F02 · The circle inside the hexagon",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 100,
      radius: 1000,
      shape: "polygon",
      width: 100,
    },
  },
  {
    explanation:
      "The same with a triangle: the circle inside a triangle is smaller, and sits lower than the middle of the box.",
    look: "A circle of radius 28.87, below the middle; “Effective radius: 28.86751 (requested 1000, reduced to fit)”.",
    title: "F02 · The circle inside the triangle",
    values: {
      anchor: "mid mid",
      corners: 3,
      height: 100,
      radius: 1000,
      shape: "polygon",
      width: 100,
    },
  },
  {
    explanation: "8 corners and a radius of 10: an octagon, slightly rounded.",
    look: "It touches all four sides of the box; “Effective radius: 10 (as requested)”.",
    title: "F02 · An octagon",
    values: {
      anchor: "mid mid",
      corners: 8,
      height: 100,
      radius: 10,
      shape: "polygon",
      width: 100,
    },
  },
  {
    explanation:
      "13 corners is refused: a polygon has 3 to 12 corners (Q19). Then type 2, and 2.5, yourself: refused too.",
    look: "The inputs are outlined in red and a message says corners must be an integer from 3 to 12.",
    title: "F02 · Too many corners",
    values: {
      anchor: "mid mid",
      corners: 13,
      height: 100,
      radius: 10,
      shape: "polygon",
      width: 100,
    },
  },
  {
    explanation: "Back to the rectangle: the same width, height and radius.",
    look: "The 100 × 100 rectangle with radius 10; the Corners field disappears.",
    title: "F02 · Back to the rectangle",
    values: {
      anchor: "mid mid",
      corners: 8,
      height: 100,
      radius: 10,
      shape: "rectangle",
      width: 100,
    },
  },
  {
    explanation:
      "A width of 250 000 is more than any screen: the interface caps it at 100 000 (Q20). The calculations themselves have no limit.",
    look: "The width input shows 100000; the rectangle runs far off the canvas.",
    title: "F02 · A size capped at 100 000",
    values: {
      anchor: "mid mid",
      corners: 8,
      height: 100,
      radius: 10,
      shape: "rectangle",
      width: 250_000,
    },
  },
];

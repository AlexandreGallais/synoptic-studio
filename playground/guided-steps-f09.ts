import type { GuidedStep } from "./guided-step";

/** The steps of the US-017 card (F09, VAL-006), one at a time. */
export const GUIDED_STEPS_F09: readonly GuidedStep[] = [
  {
    explanation:
      "F09 starts here: the rectangle is a free shape, its corners kept in a box of their own and stretched to the width and height typed.",
    look: "The same rectangle as before; under “Model”, the points of a unit square (0, 0), (1, 0), (1, 1), (0, 1), then the radii and width 120.",
    title: "F09 · The rectangle is a free shape",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 80,
      radius: 10,
      shape: "rectangle",
      width: 120,
    },
  },
  {
    explanation: "Width 0: the shape has no width any more.",
    look: "Nothing drawn; the model still holds the same unit square, with width 0.",
    title: "F09 · Width 0",
    values: { anchor: "mid mid", corners: 6, height: 80, radius: 10, shape: "rectangle", width: 0 },
  },
  {
    explanation: "Width 120 again: nothing was lost.",
    look: "Exactly the rectangle of the first F09 step.",
    title: "F09 · Back without loss",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 80,
      radius: 10,
      shape: "rectangle",
      width: 120,
    },
  },
  {
    explanation: "Height 0, the same in the other direction.",
    look: "Nothing drawn; then type 80 yourself: the rectangle comes back.",
    title: "F09 · Height 0",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 0,
      radius: 10,
      shape: "rectangle",
      width: 120,
    },
  },
  {
    explanation:
      "Points mode still works on the free shape: click Points, then point 3, and type radius 30 yourself.",
    look: "Only the bottom-right corner gets the bigger rounding; the list shows the points as drawn (point 3 at x 120, y 80).",
    title: "F09 · Points on the free shape",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 80,
      radius: 10,
      shape: "rectangle",
      width: 120,
    },
  },
];

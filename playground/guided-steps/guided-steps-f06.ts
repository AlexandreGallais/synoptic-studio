import type { GuidedStep } from "./guided-step";

/** The hexagon of the F06 steps: 100 × 100, sharp, not stretched. */
const HEXAGON = {
  anchor: "mid mid",
  corners: 6,
  height: 100,
  radius: 0,
  shape: "polygon",
  stretched: false,
  width: 100,
} as const;

/** The same hexagon, stretched to its box. */
const STRETCHED = { ...HEXAGON, stretched: true } as const;

/** A pentagon in a box too wide for equal sides: 300 × 50, stretched. */
const WIDE_PENTAGON = { ...STRETCHED, corners: 5, height: 50, width: 300 } as const;

/** The steps of the US-021 card (F06, VAL-009), one at a time. */
export const GUIDED_STEPS_F06: readonly GuidedStep[] = [
  {
    explanation:
      "F06 starts here: the regular hexagon of F02 in a 100 × 100 box. The check box “Stretched to its box” is unticked.",
    look: "The hexagon touches the left and right sides of the grey box, not its top and bottom: 6.7 of room above and below.",
    title: "F06 · The regular hexagon",
    values: HEXAGON,
  },
  {
    explanation:
      "The same hexagon with “Stretched to its box” ticked: it now touches the four sides of its box. Its sides stay equal as long as every corner keeps at least half of its usual turn (rule C).",
    look: "Six equal sides of 54.86: path “M77.42919 100 L22.57081 100 L0 50 L22.57081 0 L77.42919 0 L100 50 Z”; the anchor grid is greyed out, it has no effect.",
    title: "F06 · Stretched to its box",
    values: STRETCHED,
  },
  {
    explanation:
      "A pentagon stretched in a 300 × 50 box: too wide for equal sides without a nearly flat corner. Its sides are made equal up to that limit, then it is simply stretched.",
    look: "It touches the four sides, its apex at the top middle (150, 0); its sides are no longer equal (base 157.7, the others 80.3 and 150.6).",
    title: "F06 · A box too wide",
    values: WIDE_PENTAGON,
  },
  {
    explanation:
      "The same pentagon with radius 1000: its corners have different angles, so each one is rounded as much as it can, and not by the same amount.",
    look: "“Effective radius: mixed, shown for each point in Points mode”; in Points mode, point 4 (the apex) keeps about 42.77, the four others about 21.81.",
    title: "F06 · Different effective radii",
    values: { ...WIDE_PENTAGON, radius: 1000 },
  },
  {
    explanation: "A hexagon stretched in a tall 50 × 300 box.",
    look: "It touches the four sides; its two side corners are at mid-height (y 150).",
    title: "F06 · A tall box",
    values: { ...STRETCHED, height: 300, width: 50 },
  },
  {
    explanation: "An octagon stretched in a square box.",
    look: "The regular octagon: in a square, stretching changes nothing for it (points at 29.29 and 70.71).",
    title: "F06 · The octagon in a square",
    values: { ...STRETCHED, corners: 8 },
  },
  {
    explanation: "The stretched hexagon. Untick “Stretched to its box”.",
    look: "The regular hexagon of F02 comes back, placed by its anchor; the anchor grid is active again.",
    title: "F06 · Untick",
    values: STRETCHED,
  },
  {
    explanation: "The stretched hexagon. Choose Points mode, then click “Convert to free shape”.",
    look: "The hexagon does not move: it is now a free shape with the same six points.",
    title: "F06 · Convert",
    values: STRETCHED,
  },
  {
    explanation: "The stretched hexagon. Type 30 in Angle.",
    look: "It turns about the center of its box, like any shape; Width and Height stay 100 and 100.",
    title: "F06 · Turn it",
    values: STRETCHED,
  },
];

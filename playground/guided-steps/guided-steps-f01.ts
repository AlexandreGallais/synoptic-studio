import type { GuidedStep } from "./guided-step";

/** The nine steps of the Product Owner test card of US-003 (F01, VAL-001), one at a time. */
export const GUIDED_STEPS_F01: readonly GuidedStep[] = [
  {
    explanation:
      "This is the rectangle you get when you open the page: 120 × 80 with a radius of 10.",
    look: "Four rounded corners. Under the inputs: “Effective radius: 10 (as requested)”.",
    title: "The starting rectangle",
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
    explanation:
      "A 100 × 50 rectangle with a radius of 10: each corner is replaced by a quarter circle of radius 10.",
    look: "The four corners are rounded the same way; each curve starts 10 pixels before its corner.",
    title: "Corners rounded by 10",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 50,
      radius: 10,
      shape: "rectangle",
      width: 100,
    },
  },
  {
    explanation:
      "The rectangle is only 25 high, but you ask for 100. Two corners of 100 cannot fit on a side of 25, so the radius is reduced until they just meet: 25 / 2 = 12.5. Your 100 is kept.",
    look: "The short sides become half circles (a “pill”). The text says “Effective radius: 12.5 (requested 100, reduced to fit)”.",
    title: "A radius too big for the rectangle",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 25,
      radius: 100,
      shape: "rectangle",
      width: 100,
    },
  },
  {
    explanation:
      "A 100 × 100 square with a radius of 50, half its side: the four quarter circles join into a circle.",
    look: "A circle, and “Effective radius: 50 (as requested)”.",
    title: "A circle",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 100,
      radius: 50,
      shape: "rectangle",
      width: 100,
    },
  },
  {
    explanation:
      "The same square with a radius of 1000: far too big, so it is reduced to the most that fits, 50.",
    look: "The same circle. The text says the requested 1000 was reduced to fit, to 50.",
    title: "Much too big",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 100,
      radius: 1000,
      shape: "rectangle",
      width: 100,
    },
  },
  {
    explanation:
      "100 wide and 300 high, radius 100: the 100-pixel sides can only hold two radii of 50.",
    look: "A vertical pill, and “Effective radius: 50 (requested 100, reduced to fit)”.",
    title: "Narrow and tall",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 300,
      radius: 100,
      shape: "rectangle",
      width: 100,
    },
  },
  {
    explanation: "Now 300 wide: there is room again for the radius of 100 you asked for at step 6.",
    look: "A square with rounded corners, and “Effective radius: 100 (as requested)”: the rounding grew back to your request.",
    title: "Wider again",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 300,
      radius: 100,
      shape: "rectangle",
      width: 300,
    },
  },
  {
    explanation: "A radius of 0 means no rounding at all.",
    look: "Sharp corners, as before this feature.",
    title: "No rounding",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 300,
      radius: 0,
      shape: "rectangle",
      width: 300,
    },
  },
  {
    explanation:
      "A negative radius is refused: values must be whole numbers ≥ 0. Then type 2.5 in the radius yourself: it is refused too.",
    look: "The inputs are outlined in red, a message explains why, and the effective radius disappears.",
    title: "A wrong value",
    values: {
      anchor: "mid mid",
      corners: 6,
      height: 300,
      radius: -3,
      shape: "rectangle",
      width: 300,
    },
  },
];

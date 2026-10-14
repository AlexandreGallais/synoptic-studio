import type { GuidedStep } from "./guided-step";

/** The round shape of the F10 steps: an ellipse 120 × 80. */
const ELLIPSE = {
  anchor: "mid mid",
  corners: 6,
  height: 80,
  radius: 0,
  shape: "round",
  width: 120,
} as const;

/** The F10 options to try (VAL-012), one per step: a short list. */
export const GUIDED_STEPS_F10: readonly GuidedStep[] = [
  {
    explanation: "Round, 100 × 100.",
    look: "A circle touching its box; no Radius, no Anchor.",
    title: "F10 · Circle",
    values: { ...ELLIPSE, height: 100, width: 100 },
  },
  {
    explanation: "Round, 120 × 80.",
    look: "An ellipse touching the four sides of its box; Straight box 120 × 80.",
    title: "F10 · Ellipse",
    values: ELLIPSE,
  },
  {
    explanation: "Angle 30.",
    look: "It turns about the centre of its box.",
    title: "F10 · Turned",
    values: ELLIPSE,
  },
  {
    explanation: "Points, then point 1.",
    look: "4 points: 1 at the top, then right, bottom, left; no Handles switch, nothing to type.",
    title: "F10 · Its points",
    values: ELLIPSE,
  },
  {
    explanation: "Points, Convert to free shape; then point 1.",
    look: "Nothing moves; a free shape of 4 points, point 1 in Handles, Mirror angle and length.",
    title: "F10 · Convert to free shape",
    values: ELLIPSE,
  },
  {
    explanation: "Width 0, then Points.",
    look: "A vertical line; Convert to free shape greyed.",
    title: "F10 · A box of width 0",
    values: ELLIPSE,
  },
];

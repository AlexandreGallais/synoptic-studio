import type { GuidedStep } from "../guided-step";

/** The rectangle of the F12 steps: 120 × 80, radius 20; point 1 top left, then clockwise. */
const RECTANGLE = {
  anchor: "mid mid",
  corners: 6,
  height: 80,
  radius: 20,
  shape: "rectangle",
  width: 120,
} as const;

/** The F12 options to try, one per step: a short list, examples only where typing is hard. */
export const GUIDED_STEPS_F12: readonly GuidedStep[] = [
  {
    explanation: "Points, point 2, Handles; then Handles again.",
    look: "Sharp with handles, its radius 20 kept (effective 0); rounded again when off.",
    title: "F12 · Radius or handles",
    values: RECTANGLE,
  },
  {
    explanation: "Points, point 3, Handles; After x 90, After y 110 (x, y on the page).",
    look: "Before reads 150, 50 (mirrored); points 2 and 4 stay rounded by 20 beside the curves.",
    title: "F12 · x, y on the page, Mirror angle and length",
    values: RECTANGLE,
  },
  {
    explanation: "Points, point 3, Handles, Mode Mirror angle; After 90, 110; then Before y 40.",
    look: "After turns opposite, its own length kept: 120, 122.42641.",
    title: "F12 · Mirror angle",
    values: RECTANGLE,
  },
  {
    explanation: "Points, point 3, Handles, Mode No mirroring; After 90, 110; then Before y 40.",
    look: "After stays at 90, 110.",
    title: "F12 · No mirroring",
    values: RECTANGLE,
  },
  {
    explanation:
      "Typed as angle, length from the side, Mode No mirroring: point 1 After 330 and 40; point 2 Before 330 and 40.",
    look: "The top side is a symmetric arch; height 95. The same values give the same bend on any side.",
    title: "F12 · Angle and length from the side",
    values: RECTANGLE,
  },
  {
    explanation: "Points, point 3, Handles; After 90, 110. Then point 2, Radius 60.",
    look: "Point 2 is drawn smaller, effective 47.18996, never sharp.",
    title: "F12 · A radius too large beside a curve",
    values: RECTANGLE,
  },
  {
    explanation: "Shape Polygon, 6 corners; Points, point 1, Handles.",
    look: "It becomes a free shape without moving; only point 1 turns sharp.",
    title: "F12 · Handles on a polygon",
    values: RECTANGLE,
  },
  {
    explanation:
      "Points, point 3, Handles; After 90, 110. Then point 2, Ctrl + click point 3, Add a point in the middle.",
    look: "A point 3 at 131.25, 28.75 on the curve; the drawing does not move.",
    title: "F12 · A point added on a curve",
    values: RECTANGLE,
  },
  {
    explanation:
      "Points, point 3, Handles, Mode No mirroring; After 90, 110. Then point 4, Delete points.",
    look: "Point 4 goes; point 3 joins point 1 by one curve keeping point 3's handle.",
    title: "F12 · A point deleted next to a curve",
    values: RECTANGLE,
  },
];

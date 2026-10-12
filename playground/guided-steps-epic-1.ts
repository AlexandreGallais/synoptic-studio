import type { GuidedStep } from "./guided-step";

/** The body of a symbol in the E01 steps: a 160 × 100 rectangle rounded by 12. */
const BODY = {
  anchor: "mid mid",
  corners: 6,
  height: 100,
  radius: 12,
  shape: "rectangle",
  width: 160,
} as const;

/** A triangle standing on the bottom of its 120 × 120 box, sharp. */
const TRIANGLE = {
  anchor: "mid max",
  corners: 3,
  height: 120,
  radius: 0,
  shape: "polygon",
  width: 120,
} as const;

/** A hexagon stretched to its 160 × 100 box, sharp. */
const HEXAGON = {
  anchor: "mid mid",
  corners: 6,
  height: 100,
  radius: 0,
  shape: "polygon",
  stretched: true,
  width: 160,
} as const;

/** The user test of the epic E01 (REV-001): each step crosses several features. */
export const GUIDED_STEPS_EPIC_1: readonly GuidedStep[] = [
  {
    explanation:
      "E01's user test starts here: each step mixes several features. The body of a symbol: a 160 × 100 rectangle rounded by 12 (F01). Type 200 in Width (F09).",
    look: "The rectangle grows to 200 wide, its left side fixed; the corners keep a radius of 12, “Effective radius: 12 (as requested)”.",
    title: "E01 · A symbol body",
    values: BODY,
  },
  {
    explanation:
      "A long 160 × 40 rectangle asking for a radius of 1000 (F01): too big, so it becomes a capsule. Type 90 in Angle (F04).",
    look: "Before turning: “Effective radius: 20 (requested 1000, reduced to fit)”, half the height. After: the capsule stands up about its center and goes above the canvas; Width and Height still 160 and 40; “Straight box: x 60, y -60, width 40, height 160”.",
    title: "E01 · A capsule standing up",
    values: { ...BODY, height: 40, radius: 1000 },
  },
  {
    explanation:
      "A triangle standing on the bottom of its 120 × 120 box (F02, anchor bottom middle, F07). Click Points, point 1, then Ctrl + click point 2 (Cmd on a Mac), and type 15 in the point's Radius (F03).",
    look: "Only the two corners of the base are rounded, the apex stays sharp; the base still lies on the bottom of the box, at y 120; “Effective radius: mixed”.",
    title: "E01 · A triangle on its base",
    values: TRIANGLE,
  },
  {
    explanation:
      "A hexagon stretched to its 160 × 100 box (F06). Click Points, then “Convert to free shape” (F08); select point 1, Ctrl + click point 2, click “Add a point in the middle”, then type Y 120 and press Enter.",
    look: "Converting moves nothing; the new point 2 appears at x 80 in the middle of the base, then goes down to y 120: a point under the base, and Height becomes 120.",
    title: "E01 · A stretched hexagon made free",
    values: HEXAGON,
  },
  {
    explanation:
      "The same stretched hexagon. Type 30 in Angle (F04), then click Points and “Convert to free shape” (F08), then type 200 in Width.",
    look: "Converting a turned shape moves nothing; the wider shape keeps its left side in place: point 3 stays at x 10.71797, y 10; Width 200, the angle still 30.",
    title: "E01 · Turned, converted, widened",
    values: HEXAGON,
  },
  {
    explanation:
      "The stretched hexagon with a radius of 1000 (F06 and F01). Then type 0 in X (F04).",
    look: "Its six sides are equal, so every corner gets the same radius: “Effective radius: 50 (requested 1000, reduced to fit)”; rounding takes it away from the left and right sides of its box. With X 0, “Straight box: x 0, y 0, width 124.08793, height 100”: the drawn shape now touches the left of the canvas.",
    title: "E01 · Rounded, then placed",
    values: { ...HEXAGON, radius: 1000 },
  },
];

import type { GuidedStep } from "./guided-step";

/** The rectangle of the F12 steps: 120 × 80, sharp; point 1 top left, point 2 top right. */
const RECTANGLE = {
  anchor: "mid mid",
  corners: 6,
  height: 80,
  radius: 0,
  shape: "rectangle",
  width: 120,
} as const;

/**
 * The steps of the F12 test cards (VAL-011), one at a time: the switch, the three mirror modes,
 * then one method of typing a handle per step, the tension, and a point added or deleted on a curve.
 */
export const GUIDED_STEPS_F12: readonly GuidedStep[] = [
  {
    explanation:
      "F12 starts here: points with handles. Click Points, then point 2 (top right), then Handles. Click Handles again.",
    look: "With Handles on, point 2 is sharp: the list reads radius 20 (effective 0). With Handles off, it is rounded again, radius 20: nothing lost.",
    title: "F12 · Radius or handles",
    values: { ...RECTANGLE, radius: 20 },
  },
  {
    explanation:
      "Click Points, point 2, Handles. Mode reads Mirror angle and length, Typed as x, y on the page. Type After x 150, After y 20.",
    look: "The right side bulges out, the top side dips near point 2; Before reads x 90, y −20, the mirror. Width 133.33333, height 88.88889: the box of the curve.",
    title: "F12 · Mirror angle and length",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 2, Handles, Mode Mirror angle. Type After x 150, then Before y −20.",
    look: "After turns to x 120, y 30: opposite the handle before, keeping its own length 30.",
    title: "F12 · Mirror angle",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 2, Handles, Mode No mirroring. Type After x 150, then Before y −20.",
    look: "After stays at x 150, y 0: each handle moves alone.",
    title: "F12 · No mirroring",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 2, Handles, Typed as dx, dy from the point. Type After 30 and 20.",
    look: "The same drawing as the Mirror angle and length step; the fields read −30, −20 and 30, 20: vectors from the point.",
    title: "F12 · Typed as dx, dy",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 2, Handles, Typed as angle, length from the x axis. Type After 90 and 40.",
    look: "The handle after points straight down, 40 long (90° is down, clockwise on screen); Before reads 270 and 40.",
    title: "F12 · Typed as an angle from the x axis",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 1, Handles, Typed as angle, length from the side; After 330 and 40. Then point 2, Handles; Before 330 and 40.",
    look: "The top side is a symmetric arch, 15 above; height 95. The left and right sides bend too: each point's other handle is mirrored.",
    title: "F12 · Typed as an angle from the side",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 1, Handles, Typed as share of the side; After 30 and 20. Then point 2, Handles; Before 30 and 20.",
    look: "The top side is a symmetric arch again: each handle 30 % along the side, 20 % of it outwards; height 98.",
    title: "F12 · Typed as shares of the side",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 1, Handles. Paste C 40 -30 80 -30 120 0 in the paste field, then Paste on the side after.",
    look: "The top side arches 22.5 above, its control points those pasted; the left and right sides bend by the mirror; height 102.5.",
    title: "F12 · Paste an SVG curve",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 1, Handles. Paste cubic-bezier(0.42, 0, 0.58, 1), CSS's ease-in-out, then Paste on the side after.",
    look: "The top side becomes an S, below near point 1 and above near point 2: the easing curve laid on the side by its diagonal.",
    title: "F12 · Paste a CSS easing",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 2, Handles, Mode Smooth by a tension: a Tension field appears at 0. Type Tension 50, then 100.",
    look: "At 0 the corner is rounded smoothly from point 1 to point 3; at 50 it tightens; at 100 the sides are straight again.",
    title: "F12 · Smooth by a tension",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 1, Handles, paste C 40 -30 80 -30 120 0, Paste. Then Ctrl + click point 2 (Cmd on a Mac) and Add a point in the middle.",
    look: "Nothing moves: a new point 2 at x 60, y −22.5, on the top of the arch, its handles along the curve; width and height unchanged.",
    title: "F12 · Add a point on a curve",
    values: RECTANGLE,
  },
  {
    explanation:
      "Click Points, point 1, Handles, Mode No mirroring, paste C 40 -30 80 -30 120 0, Paste. Then click point 2 and Delete points.",
    look: "Point 2 goes; point 1 joins the old point 3 by one side that keeps point 1's handle: it rises first, then falls to the corner.",
    title: "F12 · Delete a point next to a curve",
    values: RECTANGLE,
  },
];

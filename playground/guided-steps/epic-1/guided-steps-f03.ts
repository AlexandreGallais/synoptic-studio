import type { GuidedStep } from "../guided-step";

/** The steps of the US-012 and US-013 cards, one at a time; each starts afresh. */
export const GUIDED_STEPS_F03: readonly GuidedStep[] = [
  {
    explanation:
      "F03 starts here: each corner of a shape can have its own rounding. A hexagon, no rounding. Click the large Points button yourself.",
    look: "A round handle on each corner: point 1 red (where the outline starts), point 2 green (the way it goes); a list of 6 points; the Radius field greyed, reading “Select points”.",
    title: "F03 · The points of a shape",
    values: { anchor: "mid mid", corners: 6, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation:
      "In Points mode, click the red handle, type 20; then click row 4 with Ctrl (Cmd on a Mac) held; then type 10.",
    look: "20 rounds point 1 only, the right end of the base; with points 1 and 4 selected the field reads “Mixed” (20 and 0); 10 rounds both, the others stay sharp.",
    title: "F03 · One corner, then two",
    values: { anchor: "mid mid", corners: 6, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation:
      "In Points mode, click row 2, then row 5 with Shift held; then, on the drawing, click the handle of point 6 with Shift held.",
    look: "Rows 2 to 5 turn yellow (a range); point 6 is added alone: the drawing has no range.",
    title: "F03 · Several points",
    values: { anchor: "mid mid", corners: 6, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation:
      "In Points mode, click point 1 and type 10; type 5 corners, then 6 again; then click Shape.",
    look: "The pentagon keeps the rounding of point 1; back to 6 corners the new point 6 is sharp; in Shape mode the Radius field reads “Mixed”: the corners differ.",
    title: "F03 · Radii kept by point number",
    values: { anchor: "mid mid", corners: 6, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation: "A rectangle 0 wide and 50 high has no area and is not drawn. Click Points.",
    look: "Only its handles show: the red one at the top (the green point 2 is hidden under it), a dark one at the bottom.",
    title: "F03 · Handles on top of each other",
    values: { anchor: "mid mid", corners: 6, height: 50, radius: 0, shape: "rectangle", width: 0 },
  },
];

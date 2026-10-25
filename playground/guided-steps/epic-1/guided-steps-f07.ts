import type { GuidedStep } from "../guided-step";

/** The steps of US-010's test card, one at a time, with the corner cell shown in a wide box. */
export const GUIDED_STEPS_F07: readonly GuidedStep[] = [
  {
    explanation:
      "F07 starts here. A triangle in a 100 × 100 box, radius 0. The grey rectangle is the box; the nine round buttons under “Anchor” say where the polygon sits in it.",
    look: "The triangle fills the width, with a gap above and below it; the middle button is checked.",
    title: "F07 · A triangle and its box",
    values: { anchor: "mid mid", corners: 3, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation: "Bottom middle: the triangle drops until its base lies on the bottom of the box.",
    look: "The base on the bottom edge; in the SVG output, the base at 100 and the apex at 13.39746.",
    title: "F07 · The base on the bottom",
    values: { anchor: "mid max", corners: 3, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation:
      "Bottom left: the triangle already fills the width, so left, center and right give the same drawing. The choice is still kept. Then click bottom right yourself.",
    look: "The triangle does not move; the checked button and “horizontal” in the model change.",
    title: "F07 · Kept even when nothing moves",
    values: { anchor: "min max", corners: 3, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation: "Top middle: the apex touches the top of the box, the gap is below.",
    look: "The apex on the top edge; the base at 86.60254 in the SVG output.",
    title: "F07 · The apex on the top",
    values: { anchor: "mid min", corners: 3, height: 100, radius: 0, shape: "polygon", width: 100 },
  },
  {
    explanation:
      "Radius 10: the anchor holds the sharp triangle, and the rounding works inside it without moving it.",
    look: "The rounded apex leaves the top by 10; the base and the sides stay where they were.",
    title: "F07 · Rounding does not move it",
    values: {
      anchor: "mid min",
      corners: 3,
      height: 100,
      radius: 10,
      shape: "polygon",
      width: 100,
    },
  },
  {
    explanation:
      "A square in a 100 × 50 box: it fills the height and leaves room across. Center right puts it on the right.",
    look: "A rounded square from 50 to 100 across, touching the top and the bottom.",
    title: "F07 · A square on the right",
    values: { anchor: "max mid", corners: 4, height: 50, radius: 10, shape: "polygon", width: 100 },
  },
  {
    explanation:
      "Bottom left in the same box: the room is now across, so the corner button acts as “left”. In the 100 × 100 box of step 3 it acted as “bottom”: a corner button holds both choices, and the one that counts follows the shape of the box.",
    look: "The square on the left, from 0 to 50 across.",
    title: "F07 · A corner button holds both choices",
    values: { anchor: "min max", corners: 4, height: 50, radius: 10, shape: "polygon", width: 100 },
  },
  {
    explanation:
      "The grid works with the keyboard: click a button of the grid, then press the arrow keys yourself.",
    look: "The checked button moves to the next one in reading order, and the square follows.",
    title: "F07 · With the keyboard",
    values: { anchor: "min max", corners: 4, height: 50, radius: 10, shape: "polygon", width: 100 },
  },
  {
    explanation: "Back to the rectangle: it always fills its box, so it has no anchor.",
    look: "The grid and the grey box disappear; a 100 × 50 rectangle with radius 10.",
    title: "F07 · The rectangle has no anchor",
    values: {
      anchor: "min max",
      corners: 4,
      height: 50,
      radius: 10,
      shape: "rectangle",
      width: 100,
    },
  },
];

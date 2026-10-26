import type { Point } from "editor";

/** The zoom the wheel and the pinch ask before the next frame (`REF-PENPOT-VIEWPORT-ACTIONS`). */
export type WheelZoom = {
  /** The sum of their vertical deltas, in pixels: negative toward the screen, zooming in. */
  readonly delta: number;
  /** The pointer's last offset from the canvas's top-left, in CSS pixels: the point kept. */
  readonly offset: Point;
};

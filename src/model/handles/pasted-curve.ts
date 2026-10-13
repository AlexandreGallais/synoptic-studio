import type { Point } from "../../math";

/** A curve pasted on a side: its two control points, or why it was refused. */
export type PastedCurve =
  | {
      /** The first control point, the end of the handle after the side's start. */
      readonly first: Point;
      /** The second control point, the end of the handle before the side's end. */
      readonly second: Point;
    }
  | {
      /** Why the text was refused, in a line for the page. */
      readonly refusal: string;
    };

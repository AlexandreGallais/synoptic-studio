import type { CurveCorner, Shape } from "editor";

/** A shape read from the inputs, with what the page shows of it. */
export type ShapeReading = {
  /** Corners to draw, empty when the shape is invalid. */
  readonly corners: readonly CurveCorner[];
  /** Whether the shape can enter the model. */
  readonly isValid: boolean;
  /** Message shown when it cannot. */
  readonly message: string;
  /** Requested radius of each corner, in contour order. */
  readonly radii: readonly number[];
  /** Model of the shape. */
  readonly model: Shape;
};

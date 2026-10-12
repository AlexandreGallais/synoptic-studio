import type { Corner, Shape } from "../src";

/** A shape read from the inputs, with what the page shows of it. */
export type ShapeReading = {
  /** Corners to draw, empty when the shape is invalid. */
  readonly corners: readonly Corner[];
  /** Whether the shape can enter the model. */
  readonly isValid: boolean;
  /** Message shown when it cannot. */
  readonly message: string;
  /** Radius of each corner: the one requested, or the derived one for a round shape (ADR-0036), in contour order. */
  readonly radii: readonly number[];
  /** Model of the shape. */
  readonly model: Shape;
};

import type { DrawingShape } from "editor";

/**
 * A shape the page keeps in its drawing: the shape as the model holds it, and the kind the
 * panel showed it as, since a rectangle is stored as a free shape (ADR-0032) and is loaded back
 * as a Rectangle when picked.
 */
export type KeptShape = {
  /** The kind the panel showed it as: `rectangle`, `polygon`, `round`, `free` or `text`. */
  readonly kind: string;
  /** The shape, as the model holds it. */
  readonly shape: DrawingShape;
};

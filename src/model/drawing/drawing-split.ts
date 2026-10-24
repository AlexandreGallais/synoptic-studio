/**
 * A drawing split around the shape the panel edits (F36), each part in the order drawn, so that a
 * shape picked from the middle of the drawing keeps its place.
 *
 * @template T - type of the shapes, or of what is kept of each
 */
export type DrawingSplit<T> = {
  /** The shapes drawn before the one edited, below it. */
  readonly below: readonly T[];
  /** The shape edited, alone; none when no shape is selected. */
  readonly edited: readonly T[];
  /** The shapes drawn after the one edited, above it. */
  readonly above: readonly T[];
};

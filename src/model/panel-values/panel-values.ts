/**
 * The position and size the panel shows for a shape of a drawing: the x and y of its
 * straight box, as for one shape (ADR-0035), and its own width and height, a text's those of its
 * frame.
 */
export type PanelValues = {
  /** Its own height, a text's frame's, in user units. */
  readonly height: number;
  /** Its own width, a text's frame's, in user units. */
  readonly width: number;
  /** Left of its straight box, in user units. */
  readonly x: number;
  /** Top of its straight box, in user units. */
  readonly y: number;
};

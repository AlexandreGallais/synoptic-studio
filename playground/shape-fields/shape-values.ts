/** The values typed into the panel's inputs to draw one shape alone, as Reset does. */
export type ShapeValues = {
  /** Anchor chosen, horizontal then vertical alignment (`mid max`), used by the polygon only. */
  readonly anchor: string;
  /** Number of corners typed, used by the polygon only. */
  readonly corners: number;
  /** Height typed. */
  readonly height: number;
  /** Radius typed. */
  readonly radius: number;
  /** Shape chosen: `rectangle` or `polygon`. */
  readonly shape: string;
  /** Width typed. */
  readonly width: number;
};

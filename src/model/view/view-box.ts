/** Visible region of an SVG canvas, in user units (`viewBox` attribute). */
export type ViewBox = {
  /** Height of the region. */
  readonly height: number;
  /** Width of the region. */
  readonly width: number;
  /** Abscissa of the region's top-left corner. */
  readonly x: number;
  /** Ordinate of the region's top-left corner. */
  readonly y: number;
};

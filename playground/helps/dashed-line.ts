/** A help of the layer above the drawing drawn as one line of white and black dashes. */
export type DashedLine = {
  /** The path data of the contour or box it follows, in user units. */
  readonly data: string;
  /** Its class: `<name>-light` for the white line, `<name>` for the black dashes over it. */
  readonly name: string;
  /** Its width, in screen pixels. */
  readonly width: number;
};

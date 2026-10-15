/** A color and its opacity, as a path or a stroke is filled with them, or as they are read before the model's check (ADR-0056). */
export type ColorFill = {
  /** The color, `#` and six lower-case hexadecimal digits once checked (`isColor`). */
  readonly color: string;
  /** Its opacity, an integer from 0 (invisible) to 100 %. */
  readonly opacity: number;
};

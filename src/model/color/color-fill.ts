/** A color and its opacity, as a path or a stroke is filled with them (ADR-0056). */
export type ColorFill = {
  /** The color, `#` and six lower-case hexadecimal digits. */
  readonly color: string;
  /** Its opacity, an integer from 0 (invisible) to 100 %. */
  readonly opacity: number;
};

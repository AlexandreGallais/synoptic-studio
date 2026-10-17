/** A color's fields, as its swatch names them in its `data-*` attributes. */
export type ColorField = {
  /** The id of the hexadecimal field. */
  readonly color: string;
  /** The id of the check box ticked while the color follows the text's, empty for a color that follows none (US-041). */
  readonly follows: string;
  /** The id of the opacity field. */
  readonly opacity: string;
  /** The id of the line that says why a color typed was refused. */
  readonly message: string;
};

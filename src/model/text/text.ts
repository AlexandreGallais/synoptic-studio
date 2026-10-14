import type { TextAlignment } from "./text-alignment";
import type { TextCase } from "./text-case";
import type { TextDirection } from "./text-direction";

/** A text of the model (F05, ADR-0052): its content and the options of its characters, every number an integer (ADR-0003); its lines and its block are measured from its font's metrics (`DERIV-text-width`). */
export type Text = {
  /** How its lines are aligned in its block. */
  readonly alignment: TextAlignment;
  /** Angle in degrees, an integer from 0 to 359, clockwise on screen, about the middle of its block (ADR-0035). */
  readonly angle: number;
  /** Its case, written by the library (Q36). */
  readonly textCase: TextCase;
  /** Color, `#` and six hexadecimal digits: red, green, blue (F16 brings the picker). */
  readonly color: string;
  /** What the designer typed, a line feed (U+000A) between two lines; empty, it draws nothing. */
  readonly content: string;
  /** Left to right or right to left. */
  readonly direction: TextDirection;
  /** The font family the SVG names first, any font allowed (`Inter` by default). */
  readonly font: string;
  /** The family named after it, when the reader lacks the font (`sans-serif` by default). */
  readonly fallback: string;
  /** Whether it is italic. */
  readonly isItalic: boolean;
  /** Whether a line is drawn through it. */
  readonly isStruckThrough: boolean;
  /** Whether it is underlined. */
  readonly isUnderlined: boolean;
  /** Spacing added between adjacent characters of a line, an integer in user units, negative to tighten (`REF-CSS-TEXT-3`). */
  readonly letterSpacing: number;
  /** Opacity, an integer from 0 (invisible) to 100 %. */
  readonly opacity: number;
  /** Size, the em: an integer from 1 to 100 000 user units. */
  readonly size: number;
  /** Weight, an integer from 100 (thin) to 900 (black); 400 regular, 700 bold (`REF-CSS-FONTS-4`). */
  readonly weight: number;
  /** Spacing added at each word separator (space, no-break space), an integer in user units (`REF-CSS-TEXT-3`). */
  readonly wordSpacing: number;
  /** Abscissa of the top-left of its block before it turns, in the SVG, an integer. */
  readonly x: number;
  /** Ordinate of the top-left of its block before it turns, in the SVG, an integer. */
  readonly y: number;
};

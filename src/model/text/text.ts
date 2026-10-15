import type { TextAlignment } from "./text-alignment";
import type { TextCase } from "./text-case";
import type { TextDirection } from "./text-direction";
import type { TextVerticalAlignment } from "./text-vertical-alignment";
import type { TextWrapStyle } from "./text-wrap-style";

/** A text of the model (F05, ADR-0052): its content, the options of its characters and those of its box (F14), every number an integer (ADR-0003); its lines and its block are measured from its font's metrics (`DERIV-text-width`, `DERIV-text-box`). */
export type Text = {
  /** How its lines are aligned in its block, or across its box. */
  readonly alignment: TextAlignment;
  /** Angle in degrees, an integer from 0 to 359, clockwise on screen, about the middle of its block (ADR-0035). */
  readonly angle: number;
  /** Background color behind each line, `#` and six hexadecimal digits, drawn when `hasBackground` is on. */
  readonly backgroundColor: string;
  /** Opacity of the background, an integer from 0 (invisible) to 100 %. */
  readonly backgroundOpacity: number;
  /** Height of its box, an integer from 0 to 100 000 user units, read when it sits in a box. */
  readonly boxHeight: number;
  /** Width of its box, an integer from 0 to 100 000 user units, read when it sits in a box. */
  readonly boxWidth: number;
  /** Its case, written by the library (Q36). */
  readonly textCase: TextCase;
  /** Color, `#` and six hexadecimal digits: red, green, blue (F16 brings the picker). */
  readonly color: string;
  /** What the designer typed, a line break (CR LF, CR or LF) between two lines; empty, it draws nothing. */
  readonly content: string;
  /** Left to right or right to left. */
  readonly direction: TextDirection;
  /** The font family the SVG names first, any font allowed (`Inter` by default). */
  readonly font: string;
  /** The family named after it, when the reader lacks the font (`sans-serif` by default). */
  readonly fallback: string;
  /** Whether a background is drawn behind each line, as a highlighter. */
  readonly hasBackground: boolean;
  /** Whether a text cut by its box ends with an ellipsis; on by default. */
  readonly hasEllipsis: boolean;
  /** Whether it sits in a box of `boxWidth` × `boxHeight`, its lines wrapped; otherwise it fits its content (F05). */
  readonly isInBox: boolean;
  /** Whether it is italic. */
  readonly isItalic: boolean;
  /** Whether a line is drawn through it. */
  readonly isStruckThrough: boolean;
  /** Whether its strike-through has a color of its own, `strikeThroughColor`; otherwise it follows the text's color (US-041). */
  readonly hasStrikeThroughColor: boolean;
  /** Whether its underline has a color of its own, `underlineColor`; otherwise it follows the text's color (US-041). */
  readonly hasUnderlineColor: boolean;
  /** Whether its block runs from the cap height of its first line to the baseline of its last. */
  readonly isTrimmed: boolean;
  /** Whether it is underlined. */
  readonly isUnderlined: boolean;
  /** Spacing added between adjacent characters of a line, an integer in user units, negative to tighten (`REF-CSS-TEXT-3`). */
  readonly letterSpacing: number;
  /** Distance between two baselines, an integer from 1 to 100 000 user units; 0 for the font's own (automatic). */
  readonly lineHeight: number;
  /** Most lines shown in its box, an integer from 1 to 100 000; 0 for no maximum. */
  readonly maximumLines: number;
  /** Opacity, an integer from 0 (invisible) to 100 %. */
  readonly opacity: number;
  /** Size, the em: an integer from 1 to 100 000 user units. */
  readonly size: number;
  /** Color of its strike-through, `#` and six hexadecimal digits, drawn when `hasStrikeThroughColor` is on. */
  readonly strikeThroughColor: string;
  /** Opacity of its strike-through, an integer from 0 (invisible) to 100 %, drawn when `hasStrikeThroughColor` is on. */
  readonly strikeThroughOpacity: number;
  /** Color of its underline, `#` and six hexadecimal digits, drawn when `hasUnderlineColor` is on. */
  readonly underlineColor: string;
  /** Opacity of its underline, an integer from 0 (invisible) to 100 %, drawn when `hasUnderlineColor` is on. */
  readonly underlineOpacity: number;
  /** Where its block of lines sits down its box. */
  readonly verticalAlignment: TextVerticalAlignment;
  /** Weight, an integer from 100 (thin) to 900 (black); 400 regular, 700 bold (`REF-CSS-FONTS-4`). */
  readonly weight: number;
  /** Spacing added at each word separator (space, no-break space), an integer in user units (`REF-CSS-TEXT-3`). */
  readonly wordSpacing: number;
  /** How its lines are wrapped in its box: as many words a line as fit, or balanced. */
  readonly wrapStyle: TextWrapStyle;
  /** Abscissa of the top-left of its block, or its box, before it turns, in the SVG: an integer when typed, finite and possibly fractional once a turned text has grown (ADR-0057). */
  readonly x: number;
  /** Ordinate of the top-left of its block, or its box, before it turns, in the SVG: an integer when typed, finite and possibly fractional once a turned text has grown (ADR-0057). */
  readonly y: number;
};

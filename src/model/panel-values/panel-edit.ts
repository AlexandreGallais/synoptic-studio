import type { PanelValues } from "./panel-values";
import type { TypedPanelValue } from "./typed-panel-value";

/** A value typed in one of the four fields of several shapes (F38), and what bounds a size. */
export type PanelEdit = {
  /** The field typed in: `x`, `y`, `width` or `height`. */
  readonly key: keyof PanelValues;
  /** The largest width or height the interface accepts, a larger one capped (Q20). */
  readonly maxSize: number;
  /** The value typed, relative or fixed (`parsePanelValue`). */
  readonly typed: TypedPanelValue;
};

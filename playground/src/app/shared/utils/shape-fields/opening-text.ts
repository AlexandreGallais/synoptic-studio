import { TEXT_BOX_DEFAULTS, TEXT_DECORATION_DEFAULTS } from "editor";

import type { TextFields } from "./text-fields";

/**
 * The text the page opens with, as `index.html` writes the Text tool's fields: "Pump P-101" over
 * "Running", Inter 32, regular, at (0, 0).
 */
export const OPENING_TEXT: TextFields = {
  ...TEXT_BOX_DEFAULTS,
  ...TEXT_DECORATION_DEFAULTS,
  alignment: "left",
  angle: 0,
  color: "#1e293b",
  content: "Pump P-101\nRunning",
  direction: "ltr",
  font: "Inter",
  isItalic: false,
  isStruckThrough: false,
  isUnderlined: false,
  letterSpacing: 0,
  opacity: 100,
  size: 32,
  textCase: "none",
  weight: 400,
  wordSpacing: 0,
  x: 0,
  y: 0,
};

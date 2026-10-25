import type { LineDecoration } from "./line-decoration";
import type { ColorFill } from "../color";

/** An underline or a strike-through along a line: its rectangle, and the color and the opacity it is drawn in. */
export type DrawnStroke = ColorFill & LineDecoration;

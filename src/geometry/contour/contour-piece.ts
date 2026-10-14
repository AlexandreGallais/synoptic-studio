import type { Arc } from "./arc";
import type { Cubic } from "./cubic";
import type { Segment } from "./segment";

/** Piece of an evaluated contour: a segment, a circular arc or a cubic side, told apart by `kind`. */
export type ContourPiece = Arc | Cubic | Segment;

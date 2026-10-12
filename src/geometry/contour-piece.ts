import type { Arc } from "./arc";
import type { Segment } from "./segment";

/** Piece of an evaluated contour: a segment or a circular arc, told apart by `kind`. */
export type ContourPiece = Arc | Segment;

/**
 * Where a shape sits on one axis of the room its box leaves (glossary: anchor): `min` at the
 * smallest coordinate (left or top on screen), `mid` in the middle, `max` at the largest (right
 * or bottom). SVG's `xMin`, `xMid`, `xMax` and `YMin`, `YMid`, `YMax` (`REF-SVG2-COORDS` §8.7).
 */
export type Alignment = "max" | "mid" | "min";

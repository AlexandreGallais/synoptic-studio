import type { Size } from "./size";

/**
 * The largest uniform scale that fits a span within a room, as `preserveAspectRatio`'s `meet` sets
 * the larger of the two scales to the smaller (`REF-SVG2-COORDS`, §8.2 step 7): the smaller of the
 * room's width over the span's and its height over the span's. A span of 0 on an axis bounds no
 * scale; a span of 0 on both bounds none, `Infinity`.
 *
 * Formula: min(R_w / S_w, R_h / S_h), each term over a span > 0 only.
 *
 * @kind geometry
 * @param room - the width and height to fit within, ≥ 0
 * @param span - the width and height of what is fitted, ≥ 0
 * @returns the scale, ≥ 0; `Infinity` for a span of 0 × 0
 * @see REF-SVG2-COORDS
 */
export function meetScale(room: Size, span: Size): number {
  const fits = [
    { length: span.width, side: room.width },
    { length: span.height, side: room.height },
  ]
    .filter(({ length }) => length > 0)
    .map(({ length, side }) => side / length);

  return Math.min(...fits);
}

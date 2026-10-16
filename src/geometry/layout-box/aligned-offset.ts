import { alignmentFactor } from "./alignment-factor";

import type { Alignment } from "./alignment";

/**
 * How far a span starts from the start of the room it is aligned in: the share of the room it
 * leaves that goes before it (`alignmentFactor`), none at `min`, half at `mid`, all at `max`, as
 * SVG's `preserveAspectRatio` places a viewport's content (`DERIV-regular-polygon-fit`, step 5).
 * A span longer than its room leaves a negative room: at `mid`, it overflows both ends equally, as
 * a text's block in a box too short for it (`DERIV-text-box`, step 8).
 * A room of 40 at `mid` gives 20.
 *
 * Formula: a × r, a = 0, ½ or 1 for `min`, `mid` or `max`.
 *
 * @kind geometry
 * @param room - the room the span leaves: the room's length less the span's
 * @param alignment - `min`, `mid` or `max`
 * @returns the distance from the room's start to the span's
 * @see DERIV-regular-polygon-fit
 */
export function alignedOffset(room: number, alignment: Alignment): number {
  return alignmentFactor(alignment) * room;
}

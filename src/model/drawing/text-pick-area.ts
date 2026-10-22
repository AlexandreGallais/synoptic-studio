import { polygonSegments } from "../../geometry";
import { textFrame, textFrameCorners } from "../text-layout";

import { isFlatBox } from "./is-flat-box";

import type { PickArea } from "./pick-area";
import type { Size } from "../../geometry";
import type { Text } from "../text";

/**
 * What a text offers the pointer: its frame, its block or its box when it sits in one, turned
 * with it (`textFrameCorners`), as a closed outline of four sides (`polygonSegments`), not its
 * letters; flat when the frame has no area (`textFrame`, `isFlatBox`).
 *
 * @kind domain
 * @param text - its position, its box, whether it sits in it, and its angle
 * @param block - its measured block (`textBlock`)
 * @returns its frame's corners, its sides and whether it is flat
 * @see docs/domain/interaction.md#_2-selector
 */
export function textPickArea(text: Text, block: Size): PickArea {
  const corners = textFrameCorners(text, block);

  return {
    corners,
    isFlat: isFlatBox(textFrame(text, block)),
    pieces: polygonSegments(corners),
  };
}

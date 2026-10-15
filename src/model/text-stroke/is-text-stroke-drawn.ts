import type { Text } from "../text";
import type { TextStrokeKind } from "./text-stroke-kind";

/**
 * Whether a text draws a stroke: its underline when it is underlined, its strike-through when it
 * is struck through.
 *
 * @kind domain
 * @param text - whether it is underlined and struck through
 * @param kind - the underline or the strike-through
 * @returns `true` when the text draws that stroke
 * @see docs/domain/shapes.md#text
 */
export function isTextStrokeDrawn(text: Text, kind: TextStrokeKind): boolean {
  return kind === "underline" ? text.isUnderlined : text.isStruckThrough;
}

import { readNumber, selectElement } from "../page";

/**
 * Drops the turned box kept by `writeDrawnBox` when the angle itself is refused: the next width
 * typed then places nothing. A shape refused for another value, a width being retyped, keeps it,
 * so that its turned top-left stays in place once the width is valid again (US-019).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
export function clearDrawnBox(document: Document): void {
  if (!Number.isSafeInteger(readNumber(document, "angle"))) {
    delete selectElement(document, "shape").dataset["drawnBox"];
  }
}

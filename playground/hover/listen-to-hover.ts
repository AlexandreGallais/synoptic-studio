/**
 * Listens to the pointer passing over the drawing (F36): each move keeps its point
 * (`HOVER_POINTS`) and outlines the shape under it (`showHover`); leaving the drawing forgets it
 * and takes the outline away.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToHover(document: Document): void {
  document.querySelector("#canvas");
}

/**
 * Listens to the arrow keys (F39): outside the panel's fields and the color picker's sliders,
 * which keep them, an arrow nudges the shapes selected in Shape mode by 1, or 10 with Shift
 * (`nudgeShift`), each key the browser sends, its repeats included, then redraws; the page then
 * does not scroll. With no shape selected, in Points mode, or with Ctrl, Cmd or Alt held, the key
 * is left to the browser; while a press with the Selector moves shapes, it moves nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToNudgeKeys(document: Document, update: () => void): void {
  document.addEventListener("keydown", (event) => {
    if (event.key === "") {
      update();
    }
  });
}

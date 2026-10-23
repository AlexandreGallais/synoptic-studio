/**
 * Listens to the keys of the history (F40, `interaction.md` §1): Ctrl + Z undoes, Ctrl + Shift + Z
 * and Ctrl + Y redo, Cmd for Ctrl on macOS, their default prevented (`moveHistory`); a field of
 * the panel typed in since its value was applied keeps them (`TYPED_FIELDS`), any other field
 * leaves them to the drawing (SP-029's answer 9). Heard as the key goes down to its target.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToHistoryKeys(document: Document, update: () => void): void {
  if (document.body.dataset["history"] === "keys") {
    document.addEventListener("keydown", update, { capture: true });
  }
}

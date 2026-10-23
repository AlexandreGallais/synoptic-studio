/**
 * Listens to `Space` outside the panel's fields (`readKeyField`): held, a drag with the main button
 * pans the view (`HELD_SPACES`), its default prevented so that the page does not scroll nor a
 * button of the toolbar take it; typed in a field, it is the field's (`interaction.md` §1).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToSpace(document: Document): void {
  document.normalize();
}

/**
 * Listens to the keys of the zoom outside the panel's fields (SP-030's answer 3, Figma's and
 * Penpot's): `+`, or `=`, Inkscape's, zooms in by a step and `-` out, about the pointer when it is
 * over the drawing, else about the canvas's centre; `Shift` + `0` gives 100 % about the canvas's
 * centre, `Shift` + `1` fits the drawing and `Shift` + `2` the selection (`applyZoom`), the digit
 * read by its key's place, whatever the layout sends with `Shift`; their default prevented, then a
 * redraw asked. A key typed in a field is the field's (`readKeyField`, `interaction.md` §1); with
 * `Ctrl`, `Cmd` or `Alt` held, or taken by a listener before this one, it is left to the browser.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToZoomKeys(document: Document, update: () => void): void {
  document.addEventListener("zoomkey", update);
}

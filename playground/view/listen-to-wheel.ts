/**
 * Listens to the wheel on the canvas, its default prevented, so that the page neither scrolls nor
 * zooms (`REF-MDN-WHEEL-EVENT`), its deltas in pixels (`wheelPixels`): with `Ctrl` or `Cmd` held,
 * and a pinch, which the browser sends as a wheel with `Ctrl`, the deltas gathered for the next
 * frame's zoom about the pointer (`WHEEL_ZOOMS`); alone, a pan by them, with `Shift` across
 * (`pannedView`, SP-030's answer 2).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToWheel(document: Document, update: () => void): void {
  document.addEventListener("wheelcancel", update);
}

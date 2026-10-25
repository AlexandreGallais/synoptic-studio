/**
 * Reads the swatch whose picker is open: the one marked expanded.
 *
 * @kind procedure
 * @param document - page document
 * @returns the swatch; `undefined` while the picker is closed
 */
export function readOpenSwatch(document: Document): HTMLElement | undefined {
  return document.querySelector<HTMLElement>('.swatch[aria-expanded="true"]') ?? undefined;
}

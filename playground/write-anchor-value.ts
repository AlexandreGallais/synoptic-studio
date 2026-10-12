/**
 * Checks the button of the anchor grid holding a value, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param anchor - value of the button, horizontal then vertical alignment (`mid max`)
 * @see docs/backlog/stories/E01-F07-US-010-anchor-picker-playground.md
 */
export function writeAnchorValue(document: Document, anchor: string): void {
  for (const button of document.querySelectorAll<HTMLInputElement>('input[name="anchor"]')) {
    button.checked = button.value === anchor;
  }
}

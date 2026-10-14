import { selectPageElement } from "./select-page-element";
import { typeValue } from "./type-value";

/**
 * Types a value into an input and presses Enter: its `input` then `change` events.
 *
 * @kind procedure
 * @param id - input id
 * @param value - typed text
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function typeAndEnter(id: string, value: string): void {
  typeValue(id, value);
  selectPageElement(`#${id}`).dispatchEvent(new Event("change", { bubbles: true }));
}

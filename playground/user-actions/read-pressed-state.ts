import { selectPageElement } from "./select-page-element";

/**
 * State of a toggle button, its `aria-pressed` attribute.
 *
 * @kind procedure
 * @param id - id of the button, without `#`
 * @returns `true`, `false`, or empty when the button has none
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function readPressedState(id: string): string {
  return selectPageElement(`#${id}`).getAttribute("aria-pressed") ?? "";
}

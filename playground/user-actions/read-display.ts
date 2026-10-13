import { selectPageElement } from "./select-page-element";

/**
 * Computed display of an element, so that a style rule overriding the `hidden` attribute is
 * caught: `none` when it is out of sight.
 *
 * @kind procedure
 * @param id - id of the element, without `#`
 * @returns its computed `display`
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function readDisplay(id: string): string {
  return getComputedStyle(selectPageElement(`#${id}`)).display;
}

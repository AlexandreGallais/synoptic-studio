import { selectPageElement } from "./select-page-element";

/**
 * Value of an input or a select.
 *
 * @kind procedure
 * @param id - its id, without `#`
 * @returns the value shown
 * @throws {Error} when the element has no value: neither an input nor a select
 * @see EN-021
 */
export function readValue(id: string): string {
  const found = selectPageElement(`#${id}`);

  if (found instanceof HTMLInputElement || found instanceof HTMLSelectElement) {
    return found.value;
  }

  throw new TypeError(`#${id} has no value.`);
}

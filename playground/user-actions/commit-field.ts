import { selectPageElement } from "./select-page-element";

/**
 * Commits what a field holds, as a user does by pressing Enter or leaving it: its `change` event
 * alone, nothing typed.
 *
 * @kind procedure
 * @param id - id of the field, without `#`
 */
export function commitField(id: string): void {
  selectPageElement(`#${id}`).dispatchEvent(new Event("change", { bubbles: true }));
}

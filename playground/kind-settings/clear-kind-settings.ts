import { selectElement } from "../page";

import { KIND_SETTINGS } from "./kind-settings";

/**
 * Forgets the settings kept for each kind (`saveKindSettings`), as the page opens: the next shape
 * of each kind starts with the page's values.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-1-tools
 */
export function clearKindSettings(document: Document): void {
  const { dataset } = selectElement(document, "toolbar");

  for (const kind of KIND_SETTINGS.keys()) {
    delete dataset[`${kind}Settings`];
  }
}

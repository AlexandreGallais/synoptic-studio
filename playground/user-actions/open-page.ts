import { mountPlayground } from "../mount";

import { loadPage } from "./load-page";

/**
 * Opens the playground at an address exactly as `playground/index.ts` mounts it.
 *
 * @kind procedure
 * @param search - query of the address, or empty
 */
export function openPage(search: string): void {
  loadPage(search);
  mountPlayground(document);
}

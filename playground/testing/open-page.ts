// lint-disable-next-line local/canonical-import-path -- the rule names the first folder under the common ancestor, playground/src, which has no barrel in the Angular app
import { mountPlayground } from "../src/app/shared/utils/mount";

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

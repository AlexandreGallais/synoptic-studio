import { GUIDED_STEPS } from "../guided-steps";
import { mountPlayground } from "../mount";

import { loadPage } from "./load-page";

/**
 * Opens the playground at an address, mounted with every step of the guided test.
 *
 * @kind procedure
 * @param search - query of the address, `?step=10`, or empty
 * @see EN-021
 */
export function openPage(search: string): void {
  loadPage(search);
  mountPlayground(document, GUIDED_STEPS);
}

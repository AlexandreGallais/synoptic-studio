import { openPage } from "./open-page";

/**
 * Opens the playground on a step of the guided test.
 *
 * @kind procedure
 * @param step - number of the step, from 1
 */
export function openOnStep(step: number): void {
  openPage(`?step=${String(step)}`);
}

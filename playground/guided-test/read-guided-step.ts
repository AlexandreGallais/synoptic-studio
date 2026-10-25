import { selectElement } from "../page";

/**
 * Reads which step of the guided test is shown.
 *
 * @kind procedure
 * @param document - page document
 * @returns index of the step shown, 0 before the first one
 * @see VAL-001
 */
export function readGuidedStep(document: Document): number {
  return Number(selectElement(document, "guide").dataset["step"] ?? "0");
}

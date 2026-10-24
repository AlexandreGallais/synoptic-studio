import { selectElement } from "../page";

/**
 * Makes the Text tool's fields those of a new text: no content of its own, the designer clicking,
 * then typing (US-075, superseding SP-026's `Text`), fitting it, every other option as the last
 * text left it; the box typed for the last text forgotten, so that Box chosen measures the new
 * text's (US-057).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
export function writeNewTextFields(document: Document): void {
  Object.assign(selectElement(document, "text-content"), { value: "" });
  Object.assign(selectElement(document, "text-fit"), { value: "content" });
  delete selectElement(document, "text-options").dataset["boxTyped"];
}

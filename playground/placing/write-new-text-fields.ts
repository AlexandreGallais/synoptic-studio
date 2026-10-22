import { selectElement } from "../page";

/** The content of a new text, selected so that typing replaces it (SP-026). */
const NEW_TEXT_CONTENT = "Text";

/**
 * Makes the Text tool's fields those of a new text (SP-026): its content `Text`, fitting it,
 * every other option as the last text left it; the box typed for the last text forgotten, so that
 * Box chosen measures the new text's (US-057).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
export function writeNewTextFields(document: Document): void {
  Object.assign(selectElement(document, "text-content"), { value: NEW_TEXT_CONTENT });
  Object.assign(selectElement(document, "text-fit"), { value: "content" });
  delete selectElement(document, "text-options").dataset["boxTyped"];
}

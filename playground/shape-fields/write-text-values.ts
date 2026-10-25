import { DEFAULT_DOCUMENT_OPTIONS } from "../../src";
import { selectElement } from "../page";

import { writeTextFields } from "./write-text-fields";

import type { TextFields } from "./text-fields";

/**
 * Types a text into the Text tool's fields (`writeTextFields`), and the drawing's options those of
 * a new document (`DEFAULT_DOCUMENT_OPTIONS`), Embed fonts off, as the page opens, without firing
 * any event.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 */
export function writeTextValues(document: Document, text: TextFields): void {
  const embed = selectElement(document, "embed-fonts");

  writeTextFields(document, text);

  if (embed instanceof HTMLInputElement) {
    embed.checked = DEFAULT_DOCUMENT_OPTIONS.hasEmbeddedFonts;
  }
}

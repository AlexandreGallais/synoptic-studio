import { DEFAULT_DOCUMENT_OPTIONS } from "../../src";
import { selectElement } from "../page";

import { writeTextFields } from "./write-text-fields";

import type { GuidedText } from "../guided-steps";

/**
 * Types a step's text into the Text tool's fields (`writeTextFields`), so that the step starts
 * from its own values (VAL-014), and the drawing's options those of a new document
 * (`DEFAULT_DOCUMENT_OPTIONS`), Embed fonts off, as the page opens (US-036), without firing any
 * event.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 * @see docs/backlog/stories/E15-F05-VAL-014-validate-f05.md
 */
export function writeTextStepValues(document: Document, text: GuidedText): void {
  const embed = selectElement(document, "embed-fonts");

  writeTextFields(document, text);

  if (embed instanceof HTMLInputElement) {
    embed.checked = DEFAULT_DOCUMENT_OPTIONS.hasEmbeddedFonts;
  }
}

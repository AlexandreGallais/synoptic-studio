import type { DocumentOptions } from "./document-options";

/** A new document's options: its fonts not embedded, so that no font is sent without a choice (ADR-0053). */
export const DEFAULT_DOCUMENT_OPTIONS: DocumentOptions = {
  hasEmbeddedFonts: false,
};

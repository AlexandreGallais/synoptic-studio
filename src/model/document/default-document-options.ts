import type { DocumentOptions } from "./document-options";

/** A new document's options: every option off, so that its texts are written as F05, F14 and F16 write them and no font is sent without a choice (ADR-0053). */
export const DEFAULT_DOCUMENT_OPTIONS: DocumentOptions = { hasEmbeddedFonts: false };

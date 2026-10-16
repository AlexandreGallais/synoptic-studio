import type { UnlistedFaceReason } from "../../src";

/** Why a face read is left out of the font list, installed or imported, said after its name (US-056, US-059). */
export const UNLISTED_FACE_REASONS: Readonly<Record<UnlistedFaceReason, string>> = {
  "no-latin-letters": "it lacks letters of the basic Latin alphabet (A to Z, a to z)",
  "not-embeddable": "its fsType forbids embedding it in an exported SVG",
};

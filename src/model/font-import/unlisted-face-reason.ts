import { hasBasicLatinLetters } from "./has-basic-latin-letters";

import type { FontFace } from "../font";

/** Why a face read from a file, installed or imported, is not listed (US-056, US-059): its `fsType` forbids embedding it, or it lacks a letter of the ISO basic Latin alphabet. */
export type UnlistedFaceReason = "no-latin-letters" | "not-embeddable";

/**
 * Why a face cannot be listed, the editor unable to draw, measure and embed it (US-056,
 * ADR-0061, ADR-0062: installed or imported): a file whose `fsType` does not permit embedding it whole (`FontFile.isEmbeddable`,
 * `DERIV-font-embedding`, step 3), said first, the one order of the reasons
 * (`shapes.md`, Only usable fonts listed); else a font that lacks a letter of the ISO basic
 * Latin alphabet (`hasBasicLatinLetters`). Metrics shipped without their file, Inter's, are
 * judged by their letters alone.
 *
 * @kind domain
 * @param face - the face read, its metrics and its file
 * @returns the reason; undefined for a face that can be listed
 * @see RULE-shapes-text
 */
export function unlistedFaceReason(face: FontFace): UnlistedFaceReason | undefined {
  if (face.file?.isEmbeddable === false) {
    return "not-embeddable";
  }

  return hasBasicLatinLetters(face.metrics) ? undefined : "no-latin-letters";
}

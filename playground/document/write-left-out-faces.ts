import { readFaceName } from "../fonts";
import { selectElement } from "../page";

import type { LeftOutFace, LeftOutReason } from "../../src";

/** Why a face is not embedded, as the page says it (`docs/domain/shapes.md`, Embedded fonts). */
const REASONS: Readonly<Record<LeftOutReason, string>> = {
  "family-incomplete": "another face of its family used may not be embedded",
  "generic-name": "its family is named like a generic family",
  "no-file": "its file not held",
  "not-named": "reached only as the default font, which no text names",
  "not-permitted": "its licence (fsType) does not permit it",
};

/**
 * Writes the faces the drawing's texts are measured with but that it does not embed, one line
 * each with its reason (`Inter 400: not embedded, its file not held.`), so that the designer knows
 * which fonts a reader must have; the list is hidden when it is empty.
 *
 * @kind procedure
 * @param document - page document
 * @param faces - the faces left out, from `documentFontEmbedding`
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function writeLeftOutFaces(document: Document, faces: readonly LeftOutFace[]): void {
  const list = selectElement(document, "font-left-out");

  list.replaceChildren(
    ...faces.map((face) =>
      Object.assign(document.createElement("li"), {
        textContent: `${readFaceName(face.family, face)}: not embedded, ${REASONS[face.reason]}.`,
      }),
    ),
  );

  list.hidden = faces.length === 0;
}

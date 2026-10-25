import { formatFontFaceRule } from "../io";

import { SVG_NAMESPACE } from "./svg-namespace";

import type { EmbeddedFace } from "../model";

/**
 * Creates the `<defs>` of a document's embedded fonts (ADR-0053): one `<style>` holding one
 * `@font-face` rule per face (`formatFontFaceRule`), one rule a line, in the order given; none
 * when no face is embedded, so that a document without embedded fonts is written as before.
 * Only a file written, exported or saved, holds them, never the drawing on screen (ADR-0060).
 *
 * @kind procedure
 * @param document - document owning the elements
 * @param faces - the faces embedded, from `documentFontEmbedding`
 * @returns the detached `<defs>`, or nothing
 */
export function createEmbeddedFontsElements(
  document: Document,
  faces: readonly EmbeddedFace[],
): readonly SVGDefsElement[] {
  if (faces.length === 0) {
    return [];
  }

  const defs = document.createElementNS(SVG_NAMESPACE, "defs");
  const style = document.createElementNS(SVG_NAMESPACE, "style");

  style.textContent = faces.map((face) => formatFontFaceRule(face)).join("\n");
  defs.append(style);

  return [defs];
}

import { DEFAULT_FONT, formatFontFamily, isWithinTolerance, lineWidth } from "../../src";

import type { FontFace } from "../../src";

/** The line measured in each face: Latin capitals, lower case letters, digits and a hyphen, no pair a font would join into a ligature. */
const PROBE = "Pump P-101 Running";

/** The size the line is measured at, large enough that a browser's rounding stays far below the gap between two weights of Inter (about 116 units on the probe from 400 to 500). */
const PROBE_SIZE = 1000;

/** The most the drawn width may differ from the measured one: half a unit for each of the probe's 18 characters, the most a browser rounding each advance to a whole pixel moves it. */
const TOLERANCE = 9;

/** The fallback the drawing names after Inter, so that a canvas measures what the drawing draws. */
const FALLBACK = "sans-serif";

/**
 * Reads the width a canvas gives the probe line in one of Inter's faces, its font named as the
 * drawing names it — style, weight, size, then `"Inter", sans-serif` (`REF-MDN-CANVAS-FONT`) —,
 * so that it is the face the browser draws: an installed Inter, a face imported, or the fallback.
 *
 * @kind procedure
 * @param document - page document
 * @param face - the face's weight and style
 * @returns the width the canvas measures (`TextMetrics.width`); NaN without a canvas
 * @see REF-MDN-TEXTMETRICS
 */
function readDrawnWidth(document: Document, face: Pick<FontFace, "isItalic" | "weight">): number {
  const context = document.createElement("canvas").getContext("2d");

  if (context === null) {
    return NaN;
  }

  const style = face.isItalic ? "italic " : "";

  context.font = `${style}${String(face.weight)} ${String(PROBE_SIZE)}px ${formatFontFamily(DEFAULT_FONT.name, FALLBACK)}`;

  return context.measureText(PROBE).width;
}

/**
 * Reads whether the browser draws one of Inter's shipped faces as its metrics measure it: the
 * probe's drawn width (`readDrawnWidth`) within `TOLERANCE` (`isWithinTolerance`) of its width from
 * the face's metrics (`lineWidth`, `DERIV-text-width`).
 *
 * @kind procedure
 * @param document - page document
 * @param face - one of Inter's shipped faces
 * @returns the face, kept, in a list of one; none when the browser draws another face
 * @see REF-MDN-TEXTMETRICS
 */
function readDrawnFace(document: Document, face: FontFace): readonly FontFace[] {
  const measured = lineWidth(face.metrics, PROBE, {
    letterSpacing: 0,
    size: PROBE_SIZE,
    wordSpacing: 0,
  });

  return isWithinTolerance(readDrawnWidth(document, face), measured, TOLERANCE) ? [face] : [];
}

/**
 * Reads the faces of Inter, the default font, that the browser draws as their shipped metrics
 * measure them (US-053): each face's probe line measured on a canvas, as the drawing names the
 * font, and compared with its width from the metrics. Inter missing, the browser draws its
 * fallback, Helvetica on macOS, whose widths differ; a weight Inter lacks is drawn with a nearby
 * face, whose widths differ too. A browser without a canvas cannot be asked.
 *
 * @kind procedure
 * @param document - page document
 * @returns the faces drawn as measured, none when Inter is missing; undefined when the browser cannot measure on a canvas
 * @see REF-MDN-CANVAS-FONT
 */
export function readDrawnDefaultFaces(document: Document): readonly FontFace[] | undefined {
  return document.createElement("canvas").getContext("2d") === null
    ? undefined
    : DEFAULT_FONT.faces.flatMap((face) => readDrawnFace(document, face));
}

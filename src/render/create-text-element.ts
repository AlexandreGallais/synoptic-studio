import { roundedContour } from "../geometry";
import {
  contourPiecesToPathData,
  formatSvgNumber,
  percentToOpacity,
  textToAttributes,
} from "../io";
import { decorationCorners } from "../model";

import { createPathElement } from "./create-path-element";
import { SVG_NAMESPACE } from "./svg-namespace";

import type { DrawnText, LineDecoration, PositionedLine, Text } from "../model";

/** Namespace of the `xml:` attributes (`REF-SVG2-TEXT`, §11.10.3.2). */
const XML_NAMESPACE = "http://www.w3.org/XML/1998/namespace";

/**
 * Creates the `<tspan>` of one line, at its `x` and its baseline `y`.
 *
 * @kind procedure
 * @param document - document owning the element
 * @param line - the line, placed
 * @returns the detached `<tspan>`
 * @see REF-SVG2-TEXT
 */
function createLineElement(document: Document, line: PositionedLine): SVGTSpanElement {
  const span = document.createElementNS(SVG_NAMESPACE, "tspan");

  span.setAttribute("x", formatSvgNumber(line.x));
  span.setAttribute("y", formatSvgNumber(line.y));
  span.textContent = line.content;

  return span;
}

/**
 * Creates the `<path>` of an underline or a strike-through, in the text's color and opacity.
 *
 * @kind procedure
 * @param document - document owning the element
 * @param decoration - the stroke's rectangle
 * @param text - its color and opacity
 * @returns the detached `<path>`
 * @see REF-OT-POST
 */
function createDecorationElement(
  document: Document,
  decoration: LineDecoration,
  text: Text,
): SVGPathElement {
  const contour = roundedContour(decorationCorners(decoration));
  const path = createPathElement(document, contourPiecesToPathData(contour));

  path.setAttribute("fill", text.color);
  path.setAttribute("fill-opacity", formatSvgNumber(percentToOpacity(text.opacity)));

  return path;
}

/**
 * Creates the SVG of a text (ADR-0052): a `<g>` holding one `<text>` with its attributes and one
 * positioned `<tspan>` per line, every space kept (`xml:space="preserve"`, so that a reader draws
 * the spaces the library measured), then one `<path>` rectangle per underline or strike-through.
 *
 * @kind procedure
 * @param document - document owning the elements
 * @param text - the text, for its attributes
 * @param drawn - its lines and decorations, from `textDrawing`
 * @returns the detached `<g>`
 * @see REF-SVG2-TEXT
 */
export function createTextElement(document: Document, text: Text, drawn: DrawnText): SVGGElement {
  const group = document.createElementNS(SVG_NAMESPACE, "g");
  const element = document.createElementNS(SVG_NAMESPACE, "text");

  for (const [name, value] of textToAttributes(text)) {
    element.setAttribute(name, value);
  }

  element.setAttributeNS(XML_NAMESPACE, "xml:space", "preserve");
  element.append(...drawn.lines.map((line) => createLineElement(document, line)));

  group.append(
    element,
    ...drawn.decorations.map((decoration) => createDecorationElement(document, decoration, text)),
  );

  return group;
}

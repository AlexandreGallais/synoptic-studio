import { roundedContour } from "../geometry";
import {
  contourPiecesToPathData,
  formatRotation,
  formatSubPaths,
  formatSvgNumber,
  percentToOpacity,
  textToAttributes,
} from "../io";
import { decorationCorners } from "../model";

import { createPathElement } from "./create-path-element";
import { SVG_NAMESPACE } from "./svg-namespace";

import type { DrawnText, LineDecoration, PositionedLine, Text } from "../model";

/** The fill of a `<path>`: a color and an opacity. */
type PathFill = {
  /** Its color, `#` and six hexadecimal digits. */
  readonly color: string;
  /** Its opacity, an integer from 0 (invisible) to 100 %. */
  readonly opacity: number;
};

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
 * The path data of a line's rectangle, an underline, a strike-through or a background: the
 * rectangle tool's sharp contour (`decorationCorners`), so that every rectangle is drawn by one
 * route.
 *
 * @kind format
 * @param decoration - the rectangle
 * @returns its closed sub-path, clockwise from its top-left
 * @see REF-SVG2-PATHS
 */
function decorationToPathData(decoration: LineDecoration): string {
  return contourPiecesToPathData(roundedContour(decorationCorners(decoration)));
}

/**
 * Creates a `<path>` filled with a color and an opacity from 0 to 100 %.
 *
 * @kind procedure
 * @param document - document owning the element
 * @param pathData - its `d`
 * @param fill - its color and opacity
 * @returns the detached `<path>`
 * @see REF-SVG2-PATHS
 */
function createFilledPath(document: Document, pathData: string, fill: PathFill): SVGPathElement {
  const path = createPathElement(document, pathData);

  path.setAttribute("fill", fill.color);
  path.setAttribute("fill-opacity", formatSvgNumber(percentToOpacity(fill.opacity)));

  return path;
}

/**
 * Creates the one `<path>` of a text's backgrounds, one closed sub-path per line, in the
 * background's color and opacity (`DERIV-text-box`, step 10); none when no line has one.
 *
 * @kind procedure
 * @param document - document owning the element
 * @param backgrounds - the rectangles behind the lines
 * @param text - its background's color and opacity
 * @returns the detached `<path>`, or nothing
 * @see DERIV-text-box
 */
function createBackgroundElements(
  document: Document,
  backgrounds: readonly LineDecoration[],
  text: Text,
): readonly SVGPathElement[] {
  const pathData = formatSubPaths(
    backgrounds.map((background) => decorationToPathData(background)),
  );
  const fill = { color: text.backgroundColor, opacity: text.backgroundOpacity };

  return backgrounds.length > 0 ? [createFilledPath(document, pathData, fill)] : [];
}

/**
 * Creates the SVG of a text (ADR-0052): a `<g>` holding one `<text>` with its attributes and one
 * positioned `<tspan>` per line, every space kept so that a reader draws the spaces the library
 * measured (`white-space="pre"` for SVG 2 readers, `xml:space="preserve"` for SVG 1.1 ones,
 * ADR-0054), then one `<path>` rectangle per underline or strike-through; a text with a
 * background draws it first, one `<path>` under its letters (`DERIV-text-box`, step 10). A turned
 * text's `<g>` carries a `transform` turning it about the middle of its block (ADR-0055,
 * decision 3), so that its background, underlines and strike-throughs turn with its letters; a
 * text at 0° carries none.
 *
 * @kind procedure
 * @param document - document owning the elements
 * @param text - the text, for its attributes and its angle
 * @param drawn - its middle, lines, backgrounds and decorations, from `textDrawing`
 * @returns the detached `<g>`
 * @see REF-SVG2-TEXT
 */
export function createTextElement(document: Document, text: Text, drawn: DrawnText): SVGGElement {
  const group = document.createElementNS(SVG_NAMESPACE, "g");
  const element = document.createElementNS(SVG_NAMESPACE, "text");

  for (const [name, value] of textToAttributes(text)) {
    element.setAttribute(name, value);
  }

  element.setAttribute("white-space", "pre");
  element.setAttributeNS(XML_NAMESPACE, "xml:space", "preserve");
  element.append(...drawn.lines.map((line) => createLineElement(document, line)));

  if (text.angle !== 0) {
    group.setAttribute("transform", formatRotation(text.angle, drawn.center));
  }

  const decorationFill = { color: text.color, opacity: text.opacity };

  group.append(
    ...createBackgroundElements(document, drawn.backgrounds, text),
    element,
    ...drawn.decorations.map((decoration) =>
      createFilledPath(document, decorationToPathData(decoration), decorationFill),
    ),
  );

  return group;
}

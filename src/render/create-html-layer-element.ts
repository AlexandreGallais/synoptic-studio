import {
  formatCssDeclarations,
  formatSvgNumber,
  htmlLayerToFrameStyle,
  textToHtmlStyle,
  textToParagraphStyle,
} from "../io";

import { SVG_NAMESPACE } from "./svg-namespace";
import { XHTML_NAMESPACE } from "./xhtml-namespace";

import type { CssDeclaration } from "../io";
import type { HtmlLayer, Text } from "../model";

/** A line break between two paragraphs, kept by `white-space: pre` and `pre-wrap`. */
const LINE_FEED = "\n";

/**
 * Writes the style of an XHTML element, none when it has no declaration.
 *
 * @kind procedure
 * @param element - the element, changed
 * @param declarations - its style
 * @returns the element
 * @see REF-CSS-STYLE-ATTR
 */
function writeStyle(element: HTMLElement, declarations: readonly CssDeclaration[]): HTMLElement {
  if (declarations.length > 0) {
    element.setAttribute("style", formatCssDeclarations(declarations));
  }

  return element;
}

/**
 * Creates the paragraphs of an HTML layer: one `span` each, holding its characters as text, which
 * the serializer escapes, a line feed between two (`REF-MDN-WHITE-SPACE`).
 *
 * @kind procedure
 * @param document - document owning the elements
 * @param text - its background
 * @param paragraphs - its paragraphs, in its case
 * @returns the spans and line feeds, in order
 * @see REF-MDN-WHITE-SPACE
 */
function createParagraphNodes(
  document: Document,
  text: Text,
  paragraphs: readonly string[],
): readonly Node[] {
  const style = textToParagraphStyle(text);

  return paragraphs.flatMap((paragraph, index) => {
    const span = writeStyle(document.createElementNS(XHTML_NAMESPACE, "span"), style);

    span.textContent = paragraph;

    return index === 0 ? [span] : [document.createTextNode(LINE_FEED), span];
  });
}

/**
 * Creates the HTML layer of a text (ADR-0053, decision 1): a `foreignObject` at its frame, with
 * `requiredExtensions` set to the XHTML namespace so that only a browser draws it, and
 * `overflow="visible"`, so that lines past its frame are drawn as the library draws them, holding a `div`
 * as large, which places the block down a box (`htmlLayerToFrameStyle`), holding the `div` of the
 * text's content with the CSS of its options (`textToHtmlStyle`), one `span` per paragraph.
 *
 * @kind procedure
 * @param document - document owning the elements
 * @param text - the text's options
 * @param layer - its frame, line height, line limit and paragraphs (`textHtmlLayer`)
 * @returns the detached `<foreignObject>`
 * @see REF-SVG2-EMBEDDED
 */
export function createHtmlLayerElement(
  document: Document,
  text: Text,
  layer: HtmlLayer,
): SVGForeignObjectElement {
  const foreign = document.createElementNS(SVG_NAMESPACE, "foreignObject");
  const { frame } = layer;

  const attributes = [
    ["x", frame.x],
    ["y", frame.y],
    ["width", frame.width],
    ["height", frame.height],
  ] as const;

  for (const [name, value] of attributes) {
    foreign.setAttribute(name, formatSvgNumber(value));
  }

  foreign.setAttribute("requiredExtensions", XHTML_NAMESPACE);
  foreign.setAttribute("overflow", "visible");

  const outer = writeStyle(
    document.createElementNS(XHTML_NAMESPACE, "div"),
    htmlLayerToFrameStyle(text, layer),
  );
  const content = writeStyle(
    document.createElementNS(XHTML_NAMESPACE, "div"),
    textToHtmlStyle(text, layer),
  );

  content.append(...createParagraphNodes(document, text, layer.paragraphs));
  outer.append(content);
  foreign.append(outer);

  return foreign;
}

import { readNumber, selectElement } from "../page";

import { readFieldValue } from "./read-field-value";

import type { Text, TextAlignment, TextCase, TextDirection } from "../../src";

/** The family named after the font, when a reader lacks it (F05 refinement, round 2). */
const FALLBACK = "sans-serif";

/**
 * Reads the Text tool's switches: the italic button, the underline and strike-through check
 * boxes.
 *
 * @kind procedure
 * @param document - page document
 * @returns whether each is on
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
function readTextSwitches(
  document: Document,
): Pick<Text, "isItalic" | "isStruckThrough" | "isUnderlined"> {
  return {
    isItalic: selectElement(document, "text-italic").getAttribute("aria-pressed") === "true",
    isStruckThrough: document.querySelector<HTMLInputElement>("#text-strike")?.checked === true,
    isUnderlined: document.querySelector<HTMLInputElement>("#text-underline")?.checked === true,
  };
}

/**
 * Reads the text typed in the Text tool's fields; its position is the shape's X and Y, its angle
 * 0 (a text turns with US-032). The choices are read as the page offers them, so they hold one
 * of their values.
 *
 * @kind procedure
 * @param document - page document
 * @returns the text, valid or not
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function readTextModel(document: Document): Text {
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const alignment = readFieldValue(document, "text-alignment") as TextAlignment;
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const textCase = readFieldValue(document, "text-case") as TextCase;
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const direction = readFieldValue(document, "text-direction") as TextDirection;

  return {
    alignment,
    angle: 0,
    color: readFieldValue(document, "text-color"),
    content: readFieldValue(document, "text-content"),
    direction,
    fallback: FALLBACK,
    font: readFieldValue(document, "text-font"),
    ...readTextSwitches(document),
    letterSpacing: readNumber(document, "text-letter-spacing"),
    opacity: readNumber(document, "text-opacity"),
    size: readNumber(document, "text-size"),
    textCase,
    weight: Number(readFieldValue(document, "text-weight")),
    wordSpacing: readNumber(document, "text-word-spacing"),
    x: readNumber(document, "shape-x"),
    y: readNumber(document, "shape-y"),
  };
}

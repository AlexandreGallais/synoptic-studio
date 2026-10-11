import {
  EPSILON,
  contourPiecesToPathData,
  createPathElement,
  createSvgElement,
  effectiveCornerRadius,
  formatSvgNumber,
  isValidRectangle,
  isValidRegularPolygon,
  rectangleCorners,
  regularPolygonCorners,
  roundedContour,
} from "../src";

import { createBoxElements } from "./create-box-elements";
import { GUIDED_STEPS } from "./guided-steps";
import { listenToAnchor } from "./listen-to-anchor";
import { readAnchor } from "./read-anchor";
import { writeAnchorValue } from "./write-anchor-value";

import type { GuidedStep } from "./guided-step";
import type { Corner, Rectangle, RegularPolygon } from "../src";

/** Distance from the canvas top-left corner to the shape origin, in user units (= CSS pixels). */
const MARGIN = 10;

/** Indentation of the JSON shown in the pipeline panel. */
const JSON_INDENT = 2;

/** Message shown when a value of the rectangle is not an integer ≥ 0. */
const INVALID_RECTANGLE = "Width, height and radius must be integers ≥ 0.";

/** Message shown when a value of the polygon is not allowed (Q15, Q19). */
const INVALID_POLYGON =
  "Width, height and radius must be integers ≥ 0, and corners an integer from 3 to 12.";

/** Specification of the story demonstrated by this page. */
const STORY = "docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md";

/** Largest width, height or radius the interface accepts, in pixels (Q20): a larger one is capped. */
const MAX_SIZE = 100_000;

/** Inputs capped at `MAX_SIZE`. */
const SIZE_INPUTS = ["width", "height", "radius"];

/** Number inputs of the shapes, in display order. */
const INPUTS = ["corners", "width", "height", "radius"];

/** A shape read from the inputs, with what the page shows of it. */
type ShapeReading = {
  /** Corners to draw, empty when the shape is invalid. */
  readonly corners: readonly Corner[];
  /** Whether the shape can enter the model. */
  readonly isValid: boolean;
  /** Message shown when it cannot. */
  readonly message: string;
  /** Model of the shape: integers only. */
  readonly model: Rectangle | RegularPolygon;
};

/**
 * Selects a required element of the page by its id.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `id` attribute, without `#`
 * @returns the HTML element carrying this `id`
 * @throws {Error} when the page has no element with this id
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
function selectElement(document: Document, id: string): HTMLElement {
  const element = document.querySelector<HTMLElement>(`#${id}`);

  if (element === null) {
    throw new Error(`Playground page without #${id} (${STORY}).`);
  }

  return element;
}

/**
 * Reads the number typed in an input, `NaN` when it is empty or not a number.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the input
 * @returns the typed value
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
function readNumber(document: Document, id: string): number {
  const input = selectElement(document, id);

  return input instanceof HTMLInputElement ? input.valueAsNumber : NaN;
}

/**
 * Caps the sizes typed above 100 000 at 100 000, in the inputs themselves (Q20); smaller values
 * are kept exactly. The library has no upper limit: this belongs to the interface.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F02-US-008-cap-playground-sizes.md
 */
function writeCappedSizes(document: Document): void {
  const tooLarge = SIZE_INPUTS.filter((id) => readNumber(document, id) > MAX_SIZE);

  for (const id of tooLarge) {
    writeInputValue(document, id, MAX_SIZE);
  }
}

/**
 * Reads the rectangle typed in the width, height and radius inputs.
 *
 * @kind procedure
 * @param document - page document
 * @returns the rectangle, possibly invalid (checked by `isValidRectangle`)
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
function readRectangle(document: Document): Rectangle {
  return {
    height: readNumber(document, "height"),
    radius: readNumber(document, "radius"),
    width: readNumber(document, "width"),
  };
}

/**
 * Reads the rectangle typed in the inputs, with its corners when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the rectangle
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
function readRectangleShape(document: Document): ShapeReading {
  const model = readRectangle(document);
  const isValid = isValidRectangle(model);

  return {
    corners: isValid ? rectangleCorners(model) : [],
    isValid,
    message: INVALID_RECTANGLE,
    model,
  };
}

/**
 * Reads the regular polygon typed in the inputs, with its corners when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the polygon
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function readPolygonShape(document: Document): ShapeReading {
  const anchor = readAnchor(document);
  const model = { ...readRectangle(document), anchor, corners: readNumber(document, "corners") };
  const isValid = isValidRegularPolygon(model);

  return {
    corners: isValid ? regularPolygonCorners(model) : [],
    isValid,
    message: INVALID_POLYGON,
    model,
  };
}

/**
 * Reads the shape chosen in the selector.
 *
 * @kind procedure
 * @param document - page document
 * @returns `polygon` or `rectangle`
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function readShapeKind(document: Document): string {
  const select = selectElement(document, "shape");

  return select instanceof HTMLSelectElement && select.value === "polygon"
    ? "polygon"
    : "rectangle";
}

/**
 * Writes a text into the element of the given id.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the target element
 * @param text - text to display
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
function writeText(document: Document, id: string, text: string): void {
  selectElement(document, id).textContent = text;
}

/**
 * Shows the validity of the inputs: invalid ones are outlined and a message explains why.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - shape read from the inputs
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
function setValidity(document: Document, reading: ShapeReading): void {
  for (const id of INPUTS) {
    selectElement(document, id).setAttribute("aria-invalid", String(!reading.isValid));
  }

  writeText(document, "status", reading.isValid ? "" : reading.message);
}

/**
 * Reads the size of the drawing area in CSS pixels.
 *
 * @kind procedure
 * @param document - page document
 * @returns width and height of the `#canvas` element
 * @see docs/backlog/stories/E01-F01-US-004-fixed-scale-playground.md
 */
function readCanvasSize(document: Document): { readonly height: number; readonly width: number } {
  const { height, width } = selectElement(document, "canvas").getBoundingClientRect();

  return { height, width };
}

/**
 * Shows the effective corner radius next to the requested one, and says when it was reduced to
 * fit the shape (Q8: the requested value is kept, the effective one is derived).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
function writeEffectiveRadius(document: Document, reading: ShapeReading): void {
  const effective = effectiveCornerRadius(reading.corners);
  const { radius } = reading.model;
  const isReduced = radius - effective >= EPSILON;
  const text = isReduced
    ? `Effective radius: ${formatSvgNumber(effective)} (requested ${String(radius)}, reduced to fit)`
    : `Effective radius: ${formatSvgNumber(effective)} (as requested)`;

  writeText(document, "effective", text);
}

/**
 * Draws a shape with its rounded corners at a fixed scale (1 user unit = 1 CSS pixel, US-004)
 * and shows each pipeline stage: model, contour, evaluated contour, path data.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape to draw
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
function showShape(document: Document, reading: ShapeReading): void {
  const contour = reading.corners.map((corner) => corner.point);
  const pieces = roundedContour(reading.corners);
  const pathData = contourPiecesToPathData(pieces);
  const viewBox = { ...readCanvasSize(document), x: -MARGIN, y: -MARGIN };
  const svg = createSvgElement(document, viewBox);

  svg.append(...createBoxElements(document, reading.model), createPathElement(document, pathData));
  selectElement(document, "canvas").replaceChildren(svg);
  writeText(document, "model", JSON.stringify(reading.model, undefined, JSON_INDENT));
  writeText(document, "contour", JSON.stringify(contour, undefined, JSON_INDENT));
  writeText(document, "pieces", JSON.stringify(pieces, undefined, JSON_INDENT));
  writeEffectiveRadius(document, reading);
  writeText(document, "path-data", pathData);
}

/**
 * Updates the page from the inputs: the selected shape is drawn when valid, refused otherwise;
 * the number of corners is shown for the polygon only.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function updatePlayground(document: Document): void {
  writeCappedSizes(document);

  const isPolygon = readShapeKind(document) === "polygon";
  const reading = isPolygon ? readPolygonShape(document) : readRectangleShape(document);

  selectElement(document, "corners-field").hidden = !isPolygon;
  selectElement(document, "anchor-field").hidden = !isPolygon;
  setValidity(document, reading);

  if (reading.isValid) {
    showShape(document, reading);
  } else {
    // The effective radius of the last valid rectangle would no longer match the inputs (Q8).
    writeText(document, "effective", "");
  }
}

/**
 * Types a value into an input of the page, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `corners`, `width`, `height` or `radius`
 * @param value - number to show in the input
 * @see docs/backlog/stories/E01-F01-VAL-001-validate-rectangle.md
 */
function writeInputValue(document: Document, id: string, value: number): void {
  const input = selectElement(document, id);

  if (input instanceof HTMLInputElement) {
    input.value = String(value);
  }
}

/**
 * Sets the shape selector, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - `rectangle` or `polygon`
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function writeShapeValue(document: Document, shape: string): void {
  const select = selectElement(document, "shape");

  if (select instanceof HTMLSelectElement) {
    select.value = shape;
  }
}

/**
 * Types the values of a step of the guided test into the inputs, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param step - step whose shape and numbers are typed
 * @see docs/backlog/stories/E01-F02-VAL-002-validate-polygon.md
 */
function writeStepValues(document: Document, step: GuidedStep): void {
  writeShapeValue(document, step.values.shape);
  writeAnchorValue(document, step.values.anchor);
  writeInputValue(document, "corners", step.values.corners);
  writeInputValue(document, "width", step.values.width);
  writeInputValue(document, "height", step.values.height);
  writeInputValue(document, "radius", step.values.radius);
}

/**
 * Reads which step of the guided test is shown.
 *
 * @kind procedure
 * @param document - page document
 * @returns index of the step shown, 0 before the first one
 * @see docs/backlog/stories/E01-F01-VAL-001-validate-rectangle.md
 */
function readGuidedStep(document: Document): number {
  return Number(selectElement(document, "guide").dataset["step"] ?? "0");
}

/**
 * Shows one step of the guided test and types its values, so that the drawing follows (VAL-001,
 * VAL-002).
 *
 * @kind procedure
 * @param document - page document
 * @param index - step to show, kept within the first and the last step
 * @see docs/backlog/stories/E01-F01-VAL-001-validate-rectangle.md
 */
function showGuidedStep(document: Document, index: number): void {
  const position = Math.min(Math.max(index, 0), GUIDED_STEPS.length - 1);
  const step = GUIDED_STEPS.at(position) ?? GUIDED_STEPS[0];

  if (step === undefined) {
    return;
  }

  selectElement(document, "guide").dataset["step"] = String(position);
  writeText(document, "guide-number", String(position + 1));
  writeText(document, "guide-count", String(GUIDED_STEPS.length));
  writeText(document, "guide-title", step.title);
  writeText(document, "guide-explanation", step.explanation);
  writeText(document, "guide-look", step.look);

  writeStepValues(document, step);

  updatePlayground(document);
}

/**
 * Mounts the playground: redraws on every input and window resize, wires the guided test, then
 * shows its first step.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function mountPlayground(document: Document): void {
  for (const id of INPUTS) {
    selectElement(document, id).addEventListener("input", () => {
      updatePlayground(document);
    });
  }

  selectElement(document, "shape").addEventListener("change", () => {
    updatePlayground(document);
  });

  listenToAnchor(document, () => {
    updatePlayground(document);
  });

  document.defaultView?.addEventListener("resize", () => {
    updatePlayground(document);
  });

  selectElement(document, "guide-previous").addEventListener("click", () => {
    showGuidedStep(document, readGuidedStep(document) - 1);
  });

  selectElement(document, "guide-next").addEventListener("click", () => {
    showGuidedStep(document, readGuidedStep(document) + 1);
  });

  showGuidedStep(document, 0);
}

import { fieldArrowShift } from "editor";

import { readEditedKeptShape } from "../edited-fields";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readSelectedShapes,
  writeStoredDrawing,
} from "../kept-shapes";
import { readPanelValues } from "../picking";
import { MAX_SIZE } from "../shape";

import type { KeptShape } from "../kept-shapes";
import type { PanelValues, TypedPanelValue } from "editor";

/** A value applied in a field of several shapes. */
type AppliedValue = {
  /** Whether it is an arrow's step rather than a value typed. */
  readonly isStep: boolean;
  /** The value, relative or fixed; `undefined` when the text typed is no number. */
  readonly typed: TypedPanelValue | undefined;
};

/** A value applied in a field of several shapes, a number. */
type SeveralValue = {
  /** Whether it is an arrow's step rather than a value typed. */
  readonly isStep: boolean;
  /** The field: `x`, `y`, `width` or `height`. */
  readonly key: keyof PanelValues;
  /** The value, relative or fixed. */
  readonly typed: TypedPanelValue;
};

/** The fields an arrow's step moves the shapes in as one shape's, by key. */
const STEPPED_AXES: ReadonlyMap<keyof PanelValues, "x" | "y"> = new Map([
  ["x", "x"],
  ["y", "y"],
]);

/** An arrow's step in X or Y. */
type AxisStep = {
  /** The field the arrow is pressed in: `x` or `y`. */
  readonly axis: "x" | "y";
  /** The field's step: 1 or 10 for Up, −1 or −10 for Down. */
  readonly step: number;
};

/**
 * Reads a shape selected once an arrow's step in X or Y moves it as one shape's (F46,
 * `applySteppedPosition`): landed on integers from its own straight box as last drawn
 * (`readPanelValues`, `fieldArrowShift`), X then Y set by their shifts as relative values
 * (`readEditedKeptShape`); a shape refused, or a text never measured, which has no straight box, takes
 * the step as a value typed.
 *
 * @kind procedure
 * @param document - page document
 * @param kept - a shape selected and its kind
 * @param stepped - the field the arrow is pressed in, `x` or `y`, and its step
 * @returns the shape moved and its kind
 */
function readSteppedKept(document: Document, kept: KeptShape, stepped: AxisStep): KeptShape {
  const { axis, step } = stepped;
  const [anchor] = readPanelValues(document, kept.shape);

  if (anchor === undefined) {
    return readEditedKeptShape(document, kept, {
      key: axis,
      maxSize: MAX_SIZE,
      typed: { isRelative: true, value: step },
    });
  }

  const shift = fieldArrowShift(anchor, axis, step);
  const across = { isRelative: true, value: shift.x };
  const moved = readEditedKeptShape(document, kept, { key: "x", maxSize: MAX_SIZE, typed: across });

  return readEditedKeptShape(document, moved, {
    key: "y",
    maxSize: MAX_SIZE,
    typed: { isRelative: true, value: shift.y },
  });
}

/**
 * Reads a shape selected once a value is applied in a field of several shapes: an arrow's step in
 * X or Y moves it as one shape's (`readSteppedKept`), any other value is given to it
 * (`readEditedKeptShape`).
 *
 * @kind procedure
 * @param document - page document
 * @param kept - a shape selected and its kind
 * @param value - the field, the value and whether it is an arrow's step
 * @returns the shape changed and its kind
 */
function readSeveralKept(document: Document, kept: KeptShape, value: SeveralValue): KeptShape {
  const { isStep, key, typed } = value;
  const axis = isStep ? STEPPED_AXES.get(key) : undefined;

  if (axis !== undefined) {
    return readSteppedKept(document, kept, { axis, step: typed.value });
  }

  return readEditedKeptShape(document, kept, { key, maxSize: MAX_SIZE, typed });
}

/**
 * Sets the shapes selected by a value of a field of several shapes: typed, a number
 * relative with a sign (`parsePanelValue`), or an arrow's step, relative, given to each
 * shape (`readSeveralKept`), a size taken within 0 and 100 000 as one shape's (Q20), an
 * arrow in X or Y landing each shape's own straight box on integers as one shape's,
 * the others kept as they are, in the order drawn; a text that is no number changes none. Every
 * shape is kept below while several are selected.
 *
 * @kind procedure
 * @param document - page document
 * @param key - the field: `x`, `y`, `width` or `height`
 * @param applied - the value and whether it is an arrow's step
 */
export function applySeveralValue(
  document: Document,
  key: keyof PanelValues,
  applied: AppliedValue,
): void {
  const { isStep, typed } = applied;

  if (typed === undefined) {
    return;
  }

  const selected = readSelectedShapes(document);
  const drawing = readKeptDrawing(document).map((kept, index) =>
    selected.includes(index) ? readSeveralKept(document, kept, { isStep, key, typed }) : kept,
  );

  writeStoredDrawing(document, DRAWING_KEYS.below, drawing);
}

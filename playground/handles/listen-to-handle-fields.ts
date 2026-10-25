import {
  HANDLE_MODES,
  fieldValuesOfHandle,
  freeShapeWithHandles,
  handlesAfterTyping,
  isAngleUnknown,
  typedHandle,
} from "../../src";
import { readNumber, selectElement } from "../page";
import {
  readEditedFreeShape,
  readShape,
  writeConvertedShape,
  writeStoredFreeShape,
} from "../shape";

import { readHandleFrames } from "./read-handle-frames";
import { readKeptAngle } from "./read-kept-angle";
import { readSelectedCorner } from "./read-selected-corner";
import { readTypingWay } from "./read-typing-way";
import { writeKeptAngle } from "./write-kept-angle";

import type { SelectedCorner } from "./read-selected-corner";
import type { FreeShape, HandleFrame, HandleWay, Handles, Vector } from "../../src";

/** A change to the handles of the point selected, from its handles as drawn. */
type HandlesChange = (handles: Handles, document: Document) => Handles | undefined;

/** The free shape edited and the one point selected in it. */
type EditedPoint = {
  /** The point selected, as drawn. */
  readonly selected: SelectedCorner;
  /** The free shape edited. */
  readonly shape: FreeShape;
};

/** The free shape edited, the point selected and its new handles. */
type ChangedHandles = {
  /** The new handles of the point, as drawn. */
  readonly handles: Handles;
  /** Number of the point, from 0. */
  readonly index: number;
  /** The free shape edited. */
  readonly shape: FreeShape;
};

/**
 * The free shape edited and the one point selected in it, when there are both.
 *
 * @kind procedure
 * @param document - page document
 * @returns the shape and the point, or `undefined`
 */
function readEditedPoint(document: Document): EditedPoint | undefined {
  const shape = readEditedFreeShape(document);
  const selected = readSelectedCorner(document, readShape(document));

  return shape === undefined || selected === undefined ? undefined : { selected, shape };
}

/**
 * A change applied to the point selected of the shape edited.
 *
 * @kind procedure
 * @param document - page document
 * @param change - how the handles change
 * @param edited - the shape and the point selected
 * @returns what to store, or `undefined` when nothing changes
 */
function readAppliedChange(
  document: Document,
  change: HandlesChange,
  edited: EditedPoint,
): ChangedHandles | undefined {
  const handles = change(edited.selected.corner.handles, document);

  return handles === undefined
    ? undefined
    : { handles, index: edited.selected.index, shape: edited.shape };
}

/**
 * The handles of the point selected after a change, with the shape and the point's number.
 *
 * @kind procedure
 * @param document - page document
 * @param change - how the handles change
 * @returns what to store, or `undefined` when nothing changes
 */
function readChangedHandles(document: Document, change: HandlesChange): ChangedHandles | undefined {
  const edited = readEditedPoint(document);

  return edited === undefined ? undefined : readAppliedChange(document, change, edited);
}

/**
 * Writes the handles of the one point selected after a change, then redraws: the shape becomes
 * a free shape if it was a rectangle; nothing happens without one point of a free shape.
 *
 * @kind procedure
 * @param document - page document
 * @param change - how the handles change
 * @param update - the page's redraw
 */
function writeChangedHandles(document: Document, change: HandlesChange, update: () => void): void {
  const changed = readChangedHandles(document, change);

  if (changed === undefined) {
    return;
  }

  writeStoredFreeShape(
    document,
    freeShapeWithHandles(changed.shape, changed.index, changed.handles),
  );

  update();
}

/** A value typed in a handle field, with what the move needs. */
type HandleMove = TypedField & {
  /** The angle kept for a handle of length 0 (`readKeptAngle`). */
  readonly kept: number;
  /** The value typed. */
  readonly typed: number;
  /** The way the handles are typed. */
  readonly way: HandleWay;
};

/** The field typed: which handle, and which coordinate of its end. */
type TypedField = {
  /** The coordinate typed. */
  readonly axis: "x" | "y";
  /** The handle. */
  readonly end: "after" | "before";
};

/**
 * Handles once one value of a handle is typed, in the way chosen (`typedHandle`): the other value
 * kept as it is shown, as the point's x and y; the other handle following the mode
 * (`handlesAfterTyping`); a value that is not an integer, or refused, changes nothing.
 *
 * @kind procedure
 * @param handles - handles as drawn
 * @param document - page document
 * @param field - which handle and which of its two values were typed
 * @returns the handles after the typing, or `undefined`
 */
function readTypedHandles(
  handles: Handles,
  document: Document,
  field: TypedField,
): Handles | undefined {
  const typed = readNumber(document, `handle-${field.end}-${field.axis}`);
  const reading = readShape(document);
  const selected = readSelectedCorner(document, reading);

  return selected === undefined || !Number.isSafeInteger(typed)
    ? undefined
    : readMovedHandle(handles, readHandleFrames(reading, selected)[field.end], {
        ...field,
        kept: readKeptAngle(
          document,
          field.end,
          `${String(selected.index)} ${readTypingWay(document)}`,
        ),
        typed,
        way: readTypingWay(document),
      });
}

/**
 * The two values of a handle once one is typed: the other as it is shown — computed from the
 * handle, or, for a handle of length 0 typed as an angle, the angle kept (`isAngleUnknown`).
 *
 * @kind procedure
 * @param handle - the handle typed, as drawn
 * @param frame - what it is read against
 * @param move - which value, its new value, the angle kept, the way chosen
 * @returns the two values to type
 */
function readTypedValues(
  handle: Vector,
  frame: HandleFrame,
  move: HandleMove,
): readonly [number, number] {
  const [computed, second] = fieldValuesOfHandle(move.way, handle, frame);
  const first = isAngleUnknown(move.way, handle) ? move.kept : computed;

  return move.axis === "x" ? [move.typed, second] : [first, move.typed];
}

/**
 * Handles with one value of a handle replaced by the value typed.
 *
 * @kind procedure
 * @param handles - handles as drawn
 * @param frame - what that handle is read against
 * @param move - the handle, which value, its new value, the angle kept, the way chosen
 * @returns the handles after the move, or `undefined` when the values are refused
 */
function readMovedHandle(
  handles: Handles,
  frame: HandleFrame,
  move: HandleMove,
): Handles | undefined {
  const handle = typedHandle(move.way, readTypedValues(handles[move.end], frame, move), frame);

  return handle === undefined ? undefined : handlesAfterTyping(handles, move.end, handle);
}

/**
 * Handles with the mode chosen in the list; none for a value the list does not hold.
 *
 * @kind procedure
 * @param handles - handles as drawn
 * @param document - page document
 * @returns the handles in that mode, or `undefined`
 */
function readChosenMode(handles: Handles, document: Document): Handles | undefined {
  const list = selectElement(document, "handle-mode");
  const mode = HANDLE_MODES.find(
    (known) => list instanceof HTMLSelectElement && known === list.value,
  );

  return mode === undefined ? undefined : { ...handles, mode };
}

/**
 * Keeps the first value typed for a handle, for the point selected and the way chosen, so that a
 * handle of length 0 typed from its side keeps its angle until its length is typed.
 *
 * @kind procedure
 * @param document - page document
 * @param field - which handle and which of its two values were typed
 */
function writeTypedAngle(document: Document, field: TypedField): void {
  const selected = readSelectedCorner(document, readShape(document));

  if (selected === undefined || field.axis === "y") {
    return;
  }

  const typed = readNumber(document, `handle-${field.end}-x`);

  // Its owner names the way too: an x typed on the page is never read back as an angle.
  const owner = `${String(selected.index)} ${readTypingWay(document)}`;

  writeKeptAngle(document, field.end, `${owner}|${String(typed)}`);
}

/**
 * Listens to the handles of the point selected: the switch between radius and handles, the mode,
 * the way to type them, and each handle's two values typed in that way, on "change" as the point's fields.
 *
 * @kind procedure
 * @param document - page document
 * @param update - the page's redraw
 */
export function listenToHandleFields(document: Document, update: () => void): void {
  const fields = (["before", "after"] as const).flatMap((end) =>
    (["x", "y"] as const).map((axis) => ({ axis, end })),
  );

  selectElement(document, "handles-switch").addEventListener("click", () => {
    // A regular polygon's point gets handles once the polygon is a free shape.
    writeConvertedShape(document);
    writeChangedHandles(document, (handles) => ({ ...handles, isOn: !handles.isOn }), update);
  });

  selectElement(document, "handle-mode").addEventListener("change", () => {
    writeChangedHandles(document, readChosenMode, update);
  });

  // The way to type the handles is the page's, not the model's: only the fields change.
  selectElement(document, "handle-method").addEventListener("change", update);

  for (const field of fields) {
    selectElement(document, `handle-${field.end}-${field.axis}`).addEventListener("change", () => {
      writeTypedAngle(document, field);

      writeChangedHandles(
        document,
        (handles, page) => readTypedHandles(handles, page, field),
        update,
      );
    });
  }
}

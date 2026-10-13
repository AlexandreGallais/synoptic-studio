import { HANDLE_MODES, add, freeShapeWithHandles, handlesAfterTyping, subtract } from "../../src";
import { readNumber, selectElement } from "../page";
import { readEditedFreeShape, readShape, writeStoredFreeShape } from "../shape";

import { readFieldOrigin } from "./read-field-origin";
import { readSelectedCorner } from "./read-selected-corner";

import type { SelectedCorner } from "./read-selected-corner";
import type { FreeShape, Handles, Point } from "../../src";

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
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
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
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
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
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function readChangedHandles(document: Document, change: HandlesChange): ChangedHandles | undefined {
  const edited = readEditedPoint(document);

  return edited === undefined ? undefined : readAppliedChange(document, change, edited);
}

/**
 * Writes the handles of the one point selected after a change, then redraws: the shape becomes
 * a free shape if it was a rectangle (F09); nothing happens without one point of a free shape.
 *
 * @kind procedure
 * @param document - page document
 * @param change - how the handles change
 * @param update - the page's redraw
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
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

/** The field typed: which handle, and which coordinate of its end. */
type TypedField = {
  /** The coordinate typed. */
  readonly axis: "x" | "y";
  /** The handle. */
  readonly end: "after" | "before";
};

/**
 * Handles once one coordinate of a handle's end is typed: the other coordinate kept where it is
 * drawn, as the point's x and y (F08); the vector from the point to that end, the other handle
 * following the mode (`handlesAfterTyping`); a value that is not an integer changes nothing.
 *
 * @kind procedure
 * @param handles - handles as drawn
 * @param document - page document
 * @param field - which handle and which coordinate were typed
 * @returns the handles after the typing, or `undefined`
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function readTypedHandles(
  handles: Handles,
  document: Document,
  field: TypedField,
): Handles | undefined {
  const typed = readNumber(document, `handle-${field.end}-${field.axis}`);
  const selected = readSelectedCorner(document, readShape(document));

  return selected === undefined || !Number.isSafeInteger(typed)
    ? undefined
    : readMovedEnd(handles, readFieldOrigin(document, selected.corner.point), { ...field, typed });
}

/**
 * Handles with one coordinate of a handle's end moved to the value typed.
 *
 * @kind procedure
 * @param handles - handles as drawn
 * @param origin - what the fields are counted from (`readFieldOrigin`)
 * @param move - the handle, the coordinate and its new value
 * @returns the handles after the move
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function readMovedEnd(
  handles: Handles,
  origin: Point,
  move: TypedField & { readonly typed: number },
): Handles {
  const end = { ...add(origin, handles[move.end]), [move.axis]: move.typed };

  return handlesAfterTyping(handles, move.end, subtract(end, origin));
}

/**
 * Handles with the mode chosen in the list; none for a value the list does not hold.
 *
 * @kind procedure
 * @param handles - handles as drawn
 * @param document - page document
 * @returns the handles in that mode, or `undefined`
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function readChosenMode(handles: Handles, document: Document): Handles | undefined {
  const list = selectElement(document, "handle-mode");
  const mode = HANDLE_MODES.find(
    (known) => list instanceof HTMLSelectElement && known === list.value,
  );

  return mode === undefined ? undefined : { ...handles, mode };
}

/**
 * Listens to the handles of the point selected: the switch between radius and handles, the mode,
 * and the end of each handle typed as x and y on the page, on "change" as the point's fields (US-022).
 *
 * @kind procedure
 * @param document - page document
 * @param update - the page's redraw
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
export function listenToHandleFields(document: Document, update: () => void): void {
  const fields = (["before", "after"] as const).flatMap((end) =>
    (["x", "y"] as const).map((axis) => ({ axis, end })),
  );

  selectElement(document, "handles-switch").addEventListener("click", () => {
    writeChangedHandles(document, (handles) => ({ ...handles, isOn: !handles.isOn }), update);
  });

  selectElement(document, "handle-mode").addEventListener("change", () => {
    writeChangedHandles(document, readChosenMode, update);
  });

  // The way to type the handles is the page's, not the model's: only the fields change.
  selectElement(document, "handle-method").addEventListener("change", update);

  for (const field of fields) {
    selectElement(document, `handle-${field.end}-${field.axis}`).addEventListener("change", () => {
      writeChangedHandles(
        document,
        (handles, page) => readTypedHandles(handles, page, field),
        update,
      );
    });
  }
}

import { nudgeShift, parsePanelValue } from "../../src";
import { selectElement } from "../page";
import { readFieldValue } from "../text";

import { applySeveralValue } from "./apply-several-value";
import { SEVERAL_FIELDS } from "./several-fields";

import type { PanelValues, TypedPanelValue } from "../../src";

/** The arrows that step a field of several shapes: Up adds, Down takes away. */
const FIELD_ARROWS = new Set(["ArrowDown", "ArrowUp"]);

/**
 * Reads the step an arrow pressed in a field of several shapes gives each shape's own value
 * (US-081): Up adds 1, Down takes 1 away, 10 with Shift, the arrow's step on the drawing
 * (`nudgeShift`) read up the field, as `+1` or `-10` typed; none for another key, nor with Ctrl,
 * Cmd or Alt held, which the field keeps.
 *
 * @kind procedure
 * @param event - the key pressed in the field
 * @returns the step, relative; undefined for a key that steps nothing
 * @see docs/domain/interaction.md#_2-selector
 */
function readArrowStep(event: KeyboardEvent): TypedPanelValue | undefined {
  const isLeft = [
    !FIELD_ARROWS.has(event.key),
    event.ctrlKey,
    event.metaKey,
    event.altKey,
  ].includes(true);
  const shift = nudgeShift(event.key, event.shiftKey);

  return isLeft || shift === undefined ? undefined : { isRelative: true, value: -shift.y };
}

/**
 * Applies the step of an arrow pressed in a field of several shapes (`readArrowStep`), its
 * default, the caret moved, prevented; another key is left to the field.
 *
 * @kind procedure
 * @param event - the key pressed in the field
 * @param apply - gives the step to each shape selected, then redraws (`applySeveralValue`)
 * @see docs/domain/interaction.md#_2-selector
 */
function applyArrowStep(event: KeyboardEvent, apply: (value: TypedPanelValue) => void): void {
  const step = readArrowStep(event);

  if (step === undefined) {
    return;
  }

  event.preventDefault();
  apply(step);
}

/**
 * Makes a field of several shapes set the shapes selected: a value typed applied on Enter or when
 * the field is left (`parsePanelValue`), an arrow's step at its key (`applyArrowStep`), each given
 * to every shape selected (`applySeveralValue`), then the page redrawn.
 *
 * @kind procedure
 * @param document - page document
 * @param field - the value it shows, `x`, `y`, `width` or `height`, and its id
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_2-selector
 */
function listenToSeveralField(
  document: Document,
  field: readonly [keyof PanelValues, string],
  update: () => void,
): void {
  const [key, id] = field;
  const element = selectElement(document, id);

  /**
   * Gives a value to each shape selected, then redraws.
   *
   * @kind procedure
   * @param typed - the value, relative or fixed; `undefined` when the text typed is no integer
   * @see docs/domain/interaction.md#_2-selector
   */
  function apply(typed: TypedPanelValue | undefined): void {
    applySeveralValue(document, key, typed);
    update();
  }

  element.addEventListener("change", () => {
    apply(parsePanelValue(readFieldValue(document, id)));
  });

  element.addEventListener("keydown", (event) => {
    applyArrowStep(event, apply);
  });
}

/**
 * Makes the four fields of several shapes set the shapes selected (F38): a value typed is applied
 * on Enter or when the field is left, as one shape's fields, to each shape selected
 * (`applySeveralValue`, `parsePanelValue`); Up and Down step each shape's own value by 1, or 10
 * with Shift (`applyArrowStep`, US-081); a value refused, or refused for one shape, changes none;
 * the page is then redrawn, the fields showing the values again (`listenToSeveralField`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToSeveralFields(document: Document, update: () => void): void {
  for (const field of SEVERAL_FIELDS) {
    listenToSeveralField(document, field, update);
  }
}

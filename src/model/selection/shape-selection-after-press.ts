import { shapeSelectionAfterBox } from "./shape-selection-after-box";

/**
 * The shapes selected once a press on a shape with the Selector becomes a move (F39): those
 * selected when the shape pressed is one of them, so that they all move; otherwise the shape
 * pressed alone, or with Shift added to them (`shapeSelectionAfterBox`), every shape selected then
 * moving. Ctrl is not read, as by a click.
 *
 * @kind domain
 * @param selected - indices of the shapes selected, in the order drawn, ascending, no repeat
 * @param index - index of the shape pressed (`shapeAtPoint`); −1 for none, the selection kept
 * @param isShift - whether Shift is held at the press
 * @returns the indices of the shapes moved, ascending, no repeat
 * @see RULE-interaction-2-selector
 */
export function shapeSelectionAfterPress(
  selected: readonly number[],
  index: number,
  isShift: boolean,
): readonly number[] {
  if (index < 0 || selected.includes(index)) {
    return selected;
  }

  return isShift ? shapeSelectionAfterBox(selected, [index], true) : [index];
}

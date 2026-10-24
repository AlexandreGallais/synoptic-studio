import { selectionAfterHandleClick } from "./selection-after-handle-click";

/**
 * The shapes selected after a click with the Selector (F37): Shift adds the shape under the
 * pointer, or takes it out when it is selected, a plain click selects it alone, as a vertex's
 * handle is clicked (`selectionAfterHandleClick`); on the empty drawing, Shift keeps the selection
 * and a plain click selects none. Ctrl is not read: it is kept for picking inside a group (E27).
 *
 * @kind domain
 * @param selected - indices of the shapes selected, in the order drawn, ascending, no repeat
 * @param index - index of the shape under the pointer (`shapeAtPoint`), −1 for none
 * @param isShift - whether Shift is held
 * @returns the indices selected after the click, ascending, no repeat
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapeSelectionAfterClick(
  selected: readonly number[],
  index: number,
  isShift: boolean,
): readonly number[] {
  if (index < 0) {
    return isShift ? selected : [];
  }

  // A drawing has no range: the start the vertices keep for one is not read.
  const keys = { shift: isShift, toggle: false };

  return selectionAfterHandleClick({ selected, start: undefined }, index, keys).selected;
}

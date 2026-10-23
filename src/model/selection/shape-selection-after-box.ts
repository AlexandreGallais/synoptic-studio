/**
 * The shapes selected after a selection rectangle with the Selector (F37): the shapes it takes
 * (`shapesTouchedByBox`, or `shapesEnclosedByBox` with Alt) alone; with Shift, added to the shapes
 * selected, none taken out, so that a rectangle taking none keeps the selection.
 *
 * @kind domain
 * @param selected - indices of the shapes selected, in the order drawn, ascending, no repeat
 * @param taken - indices of the shapes the rectangle takes, ascending, no repeat
 * @param isShift - whether Shift is held at the release
 * @returns the indices selected after the rectangle, ascending, no repeat
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapeSelectionAfterBox(
  selected: readonly number[],
  taken: readonly number[],
  isShift: boolean,
): readonly number[] {
  return isShift ? selected : taken;
}

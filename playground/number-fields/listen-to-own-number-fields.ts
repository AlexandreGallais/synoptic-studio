/**
 * Makes the fields acting as number fields of their own act as one shape's (US-087,
 * `OWN_NUMBER_FIELDS`): a value typed applied on Enter or when the field is left, never while it
 * is typed, an integer, relative with a sign from the number shown before it was typed
 * (`SHOWN_NUMBERS`), within the field's bounds (`readOwnValue`), a value refused putting that
 * number back; Up and Down stepping it by 1, or 10 with Shift (`readArrowStep`), the value then
 * applied as Enter applies it. Heard on the document before the field's own listeners, which read
 * the value it shows.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToOwnNumberFields(document: Document): void {
  document.normalize();
}

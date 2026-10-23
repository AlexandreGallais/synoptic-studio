/** The fields of the panel, which keep every key typed in them (`interaction.md` §1). */
const FIELDS = "input, select, textarea";

/**
 * Reads the field of the panel a key is typed in, which keeps the key (`interaction.md` §1).
 *
 * @kind procedure
 * @param event - the key pressed
 * @returns the field; undefined when the focus is outside the fields
 * @see docs/domain/interaction.md#_1-tools
 */
export function readKeyField(event: KeyboardEvent): Element | undefined {
  return event.target instanceof Element ? (event.target.closest(FIELDS) ?? undefined) : undefined;
}

/**
 * The fields of the panel, which keep every key typed in them (`interaction.md` §1): its inputs,
 * the anchor grid's radios included, and the color picker's sliders, the plane, the hue and the
 * opacity (Product Owner, 2026-10-23, AUD-041).
 */
const FIELDS = 'input, select, textarea, [role="slider"]';

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

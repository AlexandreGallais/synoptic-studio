/**
 * The fields of the panel, which keep every key typed in them
 * (`RULE-interaction-1-tools-a-key-typed-in-a-field-is-the-field-s`): its inputs,
 * the anchor grid's radios included, and the color picker's sliders, the plane, the hue and the
 * opacity (Product Owner, 2026-10-23).
 */
const FIELDS = 'input, select, textarea, [role="slider"]';

/**
 * Reads the field of the panel a key is typed in, which keeps the key
 * (`RULE-interaction-1-tools-a-key-typed-in-a-field-is-the-field-s`).
 *
 * @kind procedure
 * @param event - the key pressed
 * @returns the field; undefined when the focus is outside the fields
 */
export function readKeyField(event: KeyboardEvent): Element | undefined {
  return event.target instanceof Element ? (event.target.closest(FIELDS) ?? undefined) : undefined;
}

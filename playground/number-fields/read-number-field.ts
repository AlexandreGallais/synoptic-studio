import { NUMBER_FIELDS } from "./number-fields";

/**
 * Reads the field acting as a number field an event reached (`NUMBER_FIELDS`), whose typing
 * redraws nothing, its value applied on Enter or when it is left (US-085).
 *
 * @kind procedure
 * @param target - the target of an event of the panel
 * @returns the field; undefined for any other element
 * @see docs/domain/interaction.md#_2-selector
 */
export function readNumberField(target: EventTarget | null): Element | undefined {
  return target instanceof Element && target.matches(NUMBER_FIELDS) ? target : undefined;
}

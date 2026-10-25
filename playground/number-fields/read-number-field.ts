import { NUMBER_FIELDS } from "./number-fields";

/**
 * Reads the field acting as a number field an event reached (`NUMBER_FIELDS`), whose typing
 * redraws nothing, its value applied on Enter or when it is left.
 *
 * @kind procedure
 * @param target - the target of an event of the panel
 * @returns the field; undefined for any other element
 */
export function readNumberField(target: EventTarget | null): HTMLInputElement | undefined {
  return target instanceof HTMLInputElement && target.matches(NUMBER_FIELDS) ? target : undefined;
}

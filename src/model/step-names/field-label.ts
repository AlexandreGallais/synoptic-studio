import { FIELD_LABELS } from "./field-labels";
import { fieldWords } from "./field-words";

/**
 * The name of a step that changed one field: the panel's label of the field
 * (`FIELD_LABELS`), `isItalic` written `Italic` as its button; any other field, its words
 * (`fieldWords`), `width` written `Width`, `letterSpacing` `Letter spacing`, as the panel writes
 * them.
 *
 * @kind domain
 * @param field - a field's name, in camel case
 * @returns the panel's label of the field
 * @see RULE-interaction-7-navigation-and-history
 */
export function fieldLabel(field: string): string {
  return FIELD_LABELS.get(field) ?? fieldWords(field);
}

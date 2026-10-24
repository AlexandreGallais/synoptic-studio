import { TYPED_NUMBER_FIELDS } from "./typed-number-fields";

/**
 * Writes a text in a field acting as a number field, without firing any event, as the text it
 * shows applied, a text typed and not applied in it dropped (`TYPED_NUMBER_FIELDS`, AUD-046): the text it
 * showed before put back, a Mixed radius's empty one included, or an arrow's step, `+1`, given to
 * a field showing no number.
 *
 * @kind procedure
 * @param document - page document
 * @param id - the field's, without `#`
 * @param text - the text it shows
 * @see docs/domain/interaction.md#_2-selector
 */
export function writeFieldText(document: Document, id: string, text: string): void {
  const input = document.querySelector(`#${id}`);

  if (!(input instanceof HTMLInputElement)) {
    return;
  }

  input.value = text;
  TYPED_NUMBER_FIELDS.delete(input);
}

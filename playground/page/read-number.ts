import { selectElement } from "./select-element";

/**
 * A number as a field acting as a number field shows it (US-085), as `formatSvgNumber` writes it:
 * a minus or none, digits, then decimals or none; `+10` typed and not applied is none.
 */
const SHOWN_NUMBER = /^-?\d+(?:\.\d+)?$/u;

/**
 * Reads the number a text field acting as a number field shows (`SHOWN_NUMBER`), white space
 * around it ignored, as a number field reads its own.
 *
 * @kind procedure
 * @param input - the field, of type `text`
 * @returns the number; `NaN` for anything else, a value with a sign typed and not applied included
 * @see docs/domain/interaction.md#_2-selector
 */
function readShownNumber(input: HTMLInputElement): number {
  const text = input.value.trim();

  return SHOWN_NUMBER.test(text) ? Number(text) : NaN;
}

/**
 * Reads the number typed in an input, `NaN` when it is empty or not a number: a number field's
 * own, or what a text field acting as one shows (`readShownNumber`, US-085).
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the input
 * @returns the typed value
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function readNumber(document: Document, id: string): number {
  const input = selectElement(document, id);

  if (!(input instanceof HTMLInputElement)) {
    return NaN;
  }

  return input.type === "text" ? readShownNumber(input) : input.valueAsNumber;
}

import { readFieldValue } from "./read-field-value";

/**
 * Reads a color field of the Text tool, turned to lower case, the only case the model keeps
 * (ADR-0056): `#FF8000` typed, or written before F16, is read `#ff8000`.
 *
 * @kind procedure
 * @param document - page document
 * @param id - the field's `id` attribute, without `#`
 * @returns the color typed, in lower case, valid or not
 * @see docs/backlog/stories/E15-F16-EN-037-a-color-typed-in-hexadecimal.md
 */
export function readColorField(document: Document, id: string): string {
  return readFieldValue(document, id).toLowerCase();
}

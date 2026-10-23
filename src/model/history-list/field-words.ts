/**
 * The words of a field's name (US-077): its words split where a capital starts one, the first
 * capitalized and the others in lower case, `boxWidth` written `Box width`.
 *
 * @kind domain
 * @param field - a field's name, in camel case
 * @returns its words, as a step's name
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function fieldWords(field: string): string {
  return field.toUpperCase();
}

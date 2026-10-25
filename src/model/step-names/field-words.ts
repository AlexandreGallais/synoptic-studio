/**
 * The words of a field's name (US-077): its words split where a capital starts one, the first
 * capitalized and the others in lower case, `boxWidth` written `Box width`.
 *
 * @kind domain
 * @param field - a field's name, in camel case
 * @returns its words, as a step's name
 * @see RULE-interaction-7-navigation-and-history
 */
export function fieldWords(field: string): string {
  const words = field.replaceAll(/(?<capital>[A-Z])/gu, " $<capital>").toLowerCase();

  return `${words.slice(0, 1).toUpperCase()}${words.slice(1)}`;
}

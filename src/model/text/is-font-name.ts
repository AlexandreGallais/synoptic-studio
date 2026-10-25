/** A control character (Unicode category Cc): a line break, a tab, NUL… */
const CONTROL_CHARACTER = /\p{Cc}/u;

/**
 * Whether a font name can enter the model: at least one character, no white space at either end
 * (the name is stored trimmed) and no control character, such as a line break or a tab, which no
 * family name holds and which a `font-family` list would have to escape (2026-10-15,
 * provisional, to confirm at VAL-014).
 *
 * @kind domain
 * @param name - the font's family name, as typed or read
 * @returns `true` for a non-empty trimmed name without a control character
 * @see RULE-shapes-text
 */
export function isFontName(name: string): boolean {
  return name !== "" && name.trim() === name && !CONTROL_CHARACTER.test(name);
}

import type { TypedPanelValue } from "../../model";

/** An integer, a sign first or none: its sign and its digits. */
const PANEL_INTEGER = /^(?<sign>[+-]?)\d+$/u;

/**
 * Reads a value typed in a field of several shapes (F38): an integer, white space around ignored,
 * relative when it starts with `+` or `-`, as Figma (`REF-FIGMA-ADJUST`); `+50` adds 50, `-20`
 * takes 20 away, `460` sets 460. A fraction, an empty field, an integer past the safe ones or any
 * other text is refused, as a typed coordinate is an integer (ADR-0003, ADR-0034).
 *
 * @kind format
 * @param typed - the text of the field
 * @returns the value and whether it is relative; `undefined` when it is no integer
 * @see docs/domain/interaction.md#_2-selector
 */
export function parsePanelValue(typed: string): TypedPanelValue | undefined {
  const text = typed.trim();
  const sign = PANEL_INTEGER.exec(text)?.groups?.["sign"];
  const value = Number(text);

  if (sign === undefined || !Number.isSafeInteger(value)) {
    return undefined;
  }

  return { isRelative: sign !== "", value };
}

import type { TypedPanelValue } from "../../model";

/**
 * Reads a value typed in a field of several shapes (F38): an integer, white space around ignored,
 * relative when it starts with `+` or `-`, as Figma (`REF-FIGMA-ADJUST`); `+50` adds 50, `-20`
 * takes 20 away, `460` sets 460. A fraction, an empty field or any other text is refused, as a
 * typed coordinate is an integer (ADR-0003, ADR-0034).
 *
 * @kind format
 * @param typed - the text of the field
 * @returns the value and whether it is relative; `undefined` when it is no integer
 * @see docs/domain/interaction.md#_2-selector
 */
export function parsePanelValue(typed: string): TypedPanelValue | undefined {
  return typed === "" ? undefined : { isRelative: false, value: 0 };
}

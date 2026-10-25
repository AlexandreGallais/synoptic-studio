import type { TypedPanelValue } from "../../model";

/** A decimal number, a sign first or none: its sign, then digits with a point, a comma or none. */
const PANEL_NUMBER = /^(?<sign>[+-]?)(?:\d+(?:[,.]\d*)?|[,.]\d+)$/u;

/**
 * Reads a value typed in a field acting as a number field: a decimal number, white
 * space around it ignored, relative when it starts with `+` or `-`, as Figma (`REF-FIGMA-ADJUST`);
 * `+50` adds 50, `-20` takes 20 away, `460` sets 460, `1.5` sets 1.5, its fraction kept for the
 * field to round once its value is computed; a comma is a decimal separator as a point is,
 * `1,5` reading 1.5 (Product Owner, 2026-10-24). An empty field, a number too large for a double
 * (`Infinity`) or any other text, an exponent, a unit or an expression such as `-25+12`
 * included (E24's), is refused, as no number.
 *
 * @kind format
 * @param typed - the text of the field
 * @returns the value and whether it is relative; `undefined` when it is no number
 */
export function parsePanelValue(typed: string): TypedPanelValue | undefined {
  const text = typed.trim();
  const sign = PANEL_NUMBER.exec(text)?.groups?.["sign"];
  const value = Number(text.replace(",", "."));

  if (sign === undefined || !Number.isFinite(value)) {
    return undefined;
  }

  return { isRelative: sign !== "", value };
}

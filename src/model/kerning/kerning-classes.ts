import type { KerningRow } from "./kerning-row";

/**
 * Groups the rows of a kerning table into classes: equal rows share a class, numbered from 1 in
 * the order they first appear; a row of zeros kerns with nothing and takes class 0.
 *
 * @kind domain
 * @param rows - the kerning of each character with every other one, font units
 * @returns the class of each row, in order
 * @see DERIV-kerning-classes
 */
export function kerningClasses(rows: readonly KerningRow[]): readonly number[] {
  const keys = rows.map((row) => (row.every((value) => value === 0) ? "" : row.join(",")));
  const classes = new Map(
    [...new Set(keys.filter((key) => key !== ""))].map((key, index) => [key, index + 1]),
  );

  return keys.map((key) => classes.get(key) ?? 0);
}

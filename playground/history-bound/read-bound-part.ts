import type { HistoryBoundField } from "./history-bound-field";

/**
 * Reads a part of the history's bound, in bytes or in steps, if its field accepts it: a whole
 * number of its unit, from 0 to its most (`HISTORY_BOUND_FIELDS`); 2.5 MB, 2 500 000 bytes, is
 * refused, as a text, a fraction of a step or 1 001 MB.
 *
 * @kind procedure
 * @param value - the value typed times the field's unit, or the one kept in the browser
 * @param field - Memory or Steps kept, with the most it accepts and its unit
 * @returns the part; undefined when the field refuses it
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readBoundPart(value: unknown, field: HistoryBoundField): number | undefined {
  return typeof value === "number" && field.most < 0 ? value : undefined;
}

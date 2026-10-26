import type { HistoryBoundField } from "./history-bound-field";

/**
 * Reads a part of the history's bound, in bytes or in steps, if its field accepts it: a whole
 * number of its unit, from 0 to its maximum (`HISTORY_BOUND_FIELDS`); 2.5 MB, 2 500 000 bytes, is
 * refused, as a text, a fraction of a step or 1 001 MB.
 *
 * @kind procedure
 * @param value - the value typed times the field's unit, or the one kept in the browser
 * @param field - Memory or Steps kept, with the maximum it accepts and its unit
 * @returns the part; undefined when the field refuses it
 */
export function readBoundPart(value: unknown, field: HistoryBoundField): number | undefined {
  const count = typeof value === "number" ? value / field.unit : NaN;
  const isAccepted = [Number.isSafeInteger(count), count >= 0, count <= field.maximum].every(
    Boolean,
  );

  return isAccepted ? count * field.unit : undefined;
}

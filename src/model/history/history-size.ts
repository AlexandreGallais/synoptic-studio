import type { History } from "./history";

/**
 * The bytes a history holds beyond its present (F40): each shape a past or future state holds and
 * the present does not, counted once, its JSON text's length times 2, a string being a sequence
 * of 16-bit code units (`REF-ECMA-262-STRING-TYPE`); plus 8 bytes for each reference to a shape
 * and each index of a selection those states hold, a 64-bit slot (`REF-V8-POINTER-COMPRESSION`).
 * A count of the data, not the engine's bytes (ADR-0095, decision 4).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history measured
 * @returns its size in bytes, 0 with nothing past nor to come
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function historySize<T>(history: History<T>): number {
  return history.past.length;
}

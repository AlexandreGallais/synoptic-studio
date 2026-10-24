import type { HeldCounts } from "../held-shapes";

/**
 * What a history holds beyond its present, kept from step to step (EN-147): its size, as
 * `historySize` counts it, and how many of its past and future states hold each shape, each state
 * counting a shape once (ADR-0097).
 */
export type HistoryMeasure = {
  /** How many past and future states hold each shape, by its key (`shapeKey`). */
  readonly held: HeldCounts;
  /** The bytes the history holds beyond its present, equal to `historySize`'s. */
  readonly size: number;
};

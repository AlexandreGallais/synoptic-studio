import type { HeldCounts } from "./held-counts";

/** The held counts of a history holding no state: one level, no node (ADR-0097). */
export const EMPTY_HELD_COUNTS: HeldCounts = { depth: 1, nodes: [] };

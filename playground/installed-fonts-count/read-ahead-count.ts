import type { LeftOutFamily } from "./left-out-family";

/** What the reading ahead has counted so far. */
export type ReadAheadCount = {
  /** The installed families listed when the reading started. */
  readonly total: number;
  /** The families done: read, left out, or read meanwhile because chosen. */
  readonly done: number;
  /** The families read ahead, at least one of their faces read and listed. */
  readonly read: number;
  /** The families left out, none of their faces listed, in the browser's order. */
  readonly leftOut: readonly LeftOutFamily[];
};

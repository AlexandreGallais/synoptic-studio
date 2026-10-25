import type { LeftOutFamily } from "./left-out-family";
import type { RefusedFace } from "./refused-face";

/** What the reading ahead has counted so far. */
export type ReadAheadCount = {
  /** The installed families listed when the reading started. */
  readonly total: number;
  /** The families done: read or left out. */
  readonly done: number;
  /** The families read ahead, at least one of their faces read and listed. */
  readonly read: number;
  /** The families left out, none of their faces listed, in the browser's order. */
  readonly leftOut: readonly LeftOutFamily[];
  /** The faces refused in the families read, in the browser's order. */
  readonly refused: readonly RefusedFace[];
};

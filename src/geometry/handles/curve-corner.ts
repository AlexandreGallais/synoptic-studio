import type { Handles } from "./handles";
import type { Corner } from "../contour";

/** A corner of a free shape: its point and radius, and its handles (ADR-0046). */
export type CurveCorner = Corner & {
  /** Handles of the point, kept while it is in radius mode. */
  readonly handles: Handles;
};

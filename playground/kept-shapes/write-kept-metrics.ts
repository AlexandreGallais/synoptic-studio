import { KEPT_TEXT_METRICS } from "./kept-text-metrics";

import type { DrawingShape } from "../../src";

/**
 * Keeps for a text changed the metrics it was last drawn with (`KEPT_TEXT_METRICS`), kept by the
 * JSON of the text, so that a text whose font is no longer read is still drawn where it was moved
 * (F38, F39). A shape a tool makes has none.
 *
 * @kind procedure
 * @param before - the shape as it was
 * @param after - the shape changed
 * @see RULE-interaction-2-selector
 */
export function writeKeptMetrics(before: DrawingShape, after: DrawingShape): void {
  const metrics = KEPT_TEXT_METRICS.get(JSON.stringify(before));

  if (metrics !== undefined) {
    KEPT_TEXT_METRICS.set(JSON.stringify(after), metrics);
  }
}

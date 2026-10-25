import type { FontMetrics } from "editor";

/**
 * The metrics each text kept in the drawing was last drawn with, by the JSON of the text, so that
 * a kept text whose font's metrics are no longer read is drawn and exported as it was.
 */
export const KEPT_TEXT_METRICS = new Map<string, FontMetrics>();

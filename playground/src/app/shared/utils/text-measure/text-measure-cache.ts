import type { CachedValue } from "./cached-value";
import type { FontMetrics, MeasuredText } from "editor";

/** The lines and block a text last measured, by the metrics it was measured with: kept while every value but its position, angle, colors and opacities stays the same. */
export const TEXT_MEASURE_CACHE = new WeakMap<FontMetrics, CachedValue<MeasuredText>>();

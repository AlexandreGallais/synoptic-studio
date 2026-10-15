import type { CachedValue } from "./cached-value";
import type { FontFamily, FontMetrics } from "../../src";

/** The metrics a text was last measured with, by the session's families kept (US-048): kept while its font, fallback, weight and style stay the same. */
export const TEXT_METRICS_CACHE = new WeakMap<
  readonly FontFamily[],
  CachedValue<FontMetrics | undefined>
>();

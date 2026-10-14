import INTER from "./inter-metrics.json";

import type { FontFamily } from "./font-family";

/**
 * The default font, Inter 4.1: the metrics of its nine weights, upright and italic, read from its
 * files by `scripts/default-font-metrics.ts`; a text whose font has no metrics is measured with
 * them (ADR-0052). Inter's copyright notice and the SIL Open Font License sit beside the data, in
 * `inter-license.txt` (`REF-INTER-LICENSE`).
 */
export const DEFAULT_FONT: FontFamily = INTER;

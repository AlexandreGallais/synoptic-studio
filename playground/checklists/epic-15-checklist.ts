import { EPIC_15_COLOR_AND_FONT_LINES } from "./epic-15-color-and-font-lines";
import { EPIC_15_TEXT_LINES } from "./epic-15-text-lines";

import type { ChecklistLine } from "./checklist-line";

/**
 * E15's checklist (US-051): every text option kept after F17, one line each; none of the options
 * F17 removed.
 */
export const EPIC_15_CHECKLIST: readonly ChecklistLine[] = [
  ...EPIC_15_TEXT_LINES,
  ...EPIC_15_COLOR_AND_FONT_LINES,
];

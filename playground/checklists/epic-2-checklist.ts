import { EPIC_2_F35_LINES } from "./epic-2-f35-lines";

import type { ChecklistLine } from "./checklist-line";

/** E02's checklist: its features' lines, in the order their validation was prepared (EN-056). */
export const EPIC_2_CHECKLIST: readonly ChecklistLine[] = [...EPIC_2_F35_LINES];

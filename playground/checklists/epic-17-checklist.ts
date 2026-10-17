import { EPIC_17_F18_LINES } from "./epic-17-f18-lines";

import type { ChecklistLine } from "./checklist-line";

/** E17's checklist: its features' lines, in the order their validation was prepared (EN-056). */
export const EPIC_17_CHECKLIST: readonly ChecklistLine[] = [...EPIC_17_F18_LINES];

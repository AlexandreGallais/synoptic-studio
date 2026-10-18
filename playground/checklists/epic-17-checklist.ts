import { EPIC_17_F18_LINES } from "./epic-17-f18-lines";
import { EPIC_17_F19_LINES } from "./epic-17-f19-lines";
import { EPIC_17_F20_LINES } from "./epic-17-f20-lines";
import { EPIC_17_F21_LINES } from "./epic-17-f21-lines";

import type { ChecklistLine } from "./checklist-line";

/** E17's checklist: its features' lines, in the order their validation was prepared (EN-056). */
export const EPIC_17_CHECKLIST: readonly ChecklistLine[] = [
  ...EPIC_17_F18_LINES,
  ...EPIC_17_F19_LINES,
  ...EPIC_17_F20_LINES,
  ...EPIC_17_F21_LINES,
];

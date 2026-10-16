import { EPIC_15_CHECKLIST } from "./epic-15-checklist";

import type { ChecklistLine } from "./checklist-line";

/** The checklists the playground opens by name, `?checklist=E15` (US-051). */
export const CHECKLISTS: ReadonlyMap<string, readonly ChecklistLine[]> = new Map([
  ["E15", EPIC_15_CHECKLIST],
]);

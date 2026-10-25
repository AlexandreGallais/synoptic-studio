import { EPIC_15_CHECKLIST } from "./epic-15-checklist";
import { EPIC_17_CHECKLIST } from "./epic-17-checklist";

import type { ChecklistLine } from "./checklist-line";

/** The checklists the playground opens by name, `?checklist=E15`. */
export const CHECKLISTS: ReadonlyMap<string, readonly ChecklistLine[]> = new Map([
  ["E15", EPIC_15_CHECKLIST],
  ["E17", EPIC_17_CHECKLIST],
]);

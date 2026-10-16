/**
 * One line of an epic's checklist (US-051): what to try and what to see, ticked by the Product
 * Owner once seen.
 */
export type ChecklistLine = {
  /** The feature the line checks, `F05`. */
  readonly feature: string;
  /** What to try, in one short sentence; also the key its tick is kept under. */
  readonly todo: string;
  /** What to see once tried. */
  readonly see: string;
};

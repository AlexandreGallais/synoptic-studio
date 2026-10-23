import type { PanelValues } from "./panel-values";

/** The values the panel shows for several shapes: each common to them, or none where they differ, the field then reading Mixed. */
export type CommonPanelValues = {
  /** Their common height, `undefined` where they differ. */
  readonly height: number | undefined;
  /** Their common width, `undefined` where they differ. */
  readonly width: number | undefined;
  /** Their common x, `undefined` where they differ. */
  readonly x: number | undefined;
  /** Their common y, `undefined` where they differ. */
  readonly y: number | undefined;
};

/**
 * The values the panel shows for several shapes (F38): each one common to them, the first shape's,
 * when every shape's lies within `EPSILON` of it (`isWithinTolerance`, ADR-0003), or none where
 * they differ, the field then empty and reading Mixed, as Figma (`REF-FIGMA-ADJUST`); none for no
 * shape.
 *
 * @kind domain
 * @param values - each shape's values (`shapePanelValues`, `textPanelValues`), in the order drawn
 * @returns their common values, `undefined` where they differ (an absent value, ADR-0025)
 * @see docs/domain/interaction.md#_2-selector
 */
export function commonPanelValues(values: readonly PanelValues[]): CommonPanelValues {
  const [first] = values;

  return { height: first?.height, width: first?.width, x: first?.x, y: first?.y };
}

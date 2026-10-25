import { EPSILON, commonValue } from "../../math";

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
 * when every shape's lies within `EPSILON` of it (`commonValue`, ADR-0003), or none where
 * they differ, the field then empty and reading Mixed, as Figma (`REF-FIGMA-ADJUST`); none for no
 * shape.
 *
 * @kind domain
 * @param values - each shape's values (`shapePanelValues`, `textPanelValues`), in the order drawn
 * @returns their common values, `undefined` where they differ (an absent value, ADR-0025)
 * @see RULE-interaction-2-selector
 */
export function commonPanelValues(values: readonly PanelValues[]): CommonPanelValues {
  return {
    height: commonValue(
      values.map(({ height }) => height),
      EPSILON,
    ),
    width: commonValue(
      values.map(({ width }) => width),
      EPSILON,
    ),
    x: commonValue(
      values.map(({ x }) => x),
      EPSILON,
    ),
    y: commonValue(
      values.map(({ y }) => y),
      EPSILON,
    ),
  };
}

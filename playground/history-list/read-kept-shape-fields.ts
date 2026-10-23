import type { KeptShape } from "../kept-shapes";

/**
 * Reads the fields of a shape the page keeps that a step's name is read from (US-077): the
 * shape's own, and `kind`, the kind the panel shows it as, so that Convert to free shape, which
 * may keep the shape and change its kind alone, is named `Convert` (`stepName`).
 *
 * @kind procedure
 * @param kept - a shape as the page keeps it
 * @returns its shape's fields and its kind
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readKeptShapeFields(kept: KeptShape): Readonly<Record<string, unknown>> {
  return { ...kept.shape, kind: kept.kind };
}

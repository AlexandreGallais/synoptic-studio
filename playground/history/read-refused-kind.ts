import { readShapeKind } from "../page";

/**
 * Reads the kind of the shape the panel edits when it is refused (F40): a value out of its range,
 * a text not drawn, so that the page keeps no shape edited while the panel still shows one; no
 * step is taken meanwhile, the drawing kept without it being no change the designer made.
 *
 * @kind procedure
 * @param document - page document, just redrawn
 * @returns `rectangle`, `polygon`, `round`, `free` or `text`; undefined for a shape drawn, or
 * none or several selected
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readRefusedKind(document: Document): string | undefined {
  const kind = readShapeKind(document);

  return kind === "refused" ? kind : undefined;
}

import { readShapeKind, selectElement, writeInputValue } from "../page";

import { TOOL_PLACEMENT } from "./tool-placement";

/** A placement kept by `saveToolPlacement`. */
type KeptPlacement = {
  /** The X, Y and angle fields, as they were. */
  readonly fields: readonly string[];
  /** The shape's position, `#shape`'s `data-x` and `data-y`, when it had one. */
  readonly position: { readonly x?: string; readonly y?: string };
};

/**
 * Reads the placement kept for the kind chosen; for a kind not yet placed, X 0, Y 0, straight.
 *
 * @kind procedure
 * @param document - page document
 * @param isText - whether the Text tool is chosen
 * @returns the fields and the position to show
 * @see docs/backlog/stories/E15-F05-US-032-turn-a-text.md
 */
function readKeptPlacement(document: Document, isText: boolean): KeptPlacement {
  const { dataset } = selectElement(document, "placement");

  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by saveToolPlacement or TOOL_PLACEMENT.start only
  return JSON.parse(
    dataset[isText ? TOOL_PLACEMENT.textKey : TOOL_PLACEMENT.shapeKey] ?? TOOL_PLACEMENT.start,
  ) as KeptPlacement;
}

/**
 * Gives the Text tool and the shapes each their own X, Y and angle (US-032): when the other kind
 * is chosen, its fields and the shape's position come back as `saveToolPlacement` kept them, a
 * kind not yet placed starting at X 0, Y 0, straight. A text's angle never turns the shape, nor
 * the other way round; choosing a shape after a shape changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param kind - the kind about to be chosen
 * @see docs/backlog/stories/E15-F05-US-032-turn-a-text.md
 */
export function writeToolPlacement(document: Document, kind: string): void {
  const isText = kind === "text";

  if ((readShapeKind(document) === "text") === isText) {
    return;
  }

  const placement = readKeptPlacement(document, isText);

  for (const [index, id] of TOOL_PLACEMENT.fields.entries()) {
    writeInputValue(document, id, Number(placement.fields[index]));
  }

  Object.assign(selectElement(document, "shape").dataset, placement.position);
}

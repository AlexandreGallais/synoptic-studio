/**
 * What the Text tool and the shapes each keep of their placement (US-032): the X, Y and angle
 * fields, and the shape's position (`#shape`'s `data-x` and `data-y`, which choosing a tool
 * clears), kept as JSON on the placement fieldset under one `data-*` key for the text and one for
 * the shapes.
 */
export const TOOL_PLACEMENT = {
  /** The fields of a placement, in order. */
  fields: ["shape-x", "shape-y", "angle"],
  /** The `data-*` key keeping the shapes' placement. */
  shapeKey: "shapePlacement",
  /** A tool not yet placed: X 0, Y 0, straight, no position kept. */
  start: JSON.stringify({ fields: ["0", "0", "0"], position: {} }),
  /** The `data-*` key keeping the text's placement. */
  textKey: "textPlacement",
} as const;

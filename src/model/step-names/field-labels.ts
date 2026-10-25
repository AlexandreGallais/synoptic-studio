/**
 * The panel's label of each field of a shape whose words (`fieldWords`) are not the label:
 * a text's switches named as their boxes, its box's size as its fields, a polygon's radii
 * as Radius; the label's unit or hint in brackets left out, `Max lines (0 none)` written `Max
 * lines`.
 */
export const FIELD_LABELS: ReadonlyMap<string, string> = new Map([
  ["boxHeight", "Height"],
  ["boxWidth", "Width"],
  ["hasBackground", "Background"],
  ["hasEllipsis", "Ellipsis"],
  ["hasStrikeThroughColor", "Strike-through follows the text's color"],
  ["hasUnderlineColor", "Underline follows the text's color"],
  ["isInBox", "Fit"],
  ["isItalic", "Italic"],
  ["isStruckThrough", "Strike-through"],
  ["isTrimmed", "Trim"],
  ["isUnderlined", "Underline"],
  ["maximumLines", "Max lines"],
  ["radii", "Radius"],
  ["stretched", "Stretched to its box"],
  ["strikeThroughColor", "Strike-through color"],
  ["strikeThroughOpacity", "Strike-through opacity"],
  ["textCase", "Case"],
]);

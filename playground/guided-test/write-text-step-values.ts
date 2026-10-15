import { selectElement, writeInputValue } from "../page";

import type { GuidedText } from "../guided-steps";

/**
 * Writes a value into a text field, a text area or a choice of the page, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param id - its `id` attribute, without `#`
 * @param value - the value written
 * @see docs/backlog/stories/E15-F05-VAL-014-validate-f05.md
 */
function writeFieldValue(document: Document, id: string, value: string): void {
  // An input, a text area and a select each hold their value in `value`.
  Object.assign(selectElement(document, id), { value });
}

/**
 * Ticks or unticks a check box of the page, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param id - its `id` attribute, without `#`
 * @param isChecked - whether it is ticked
 * @see docs/backlog/stories/E15-F05-VAL-014-validate-f05.md
 */
function writeCheckedValue(document: Document, id: string, isChecked: boolean): void {
  const box = selectElement(document, id);

  if (box instanceof HTMLInputElement) {
    box.checked = isChecked;
  }
}

/**
 * Types a step's text into the Text tool's fields and its X, Y and angle into the placement
 * fields, without firing any event; the placements each tool kept go, so that the step starts
 * from its own values (VAL-014).
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 * @see docs/backlog/stories/E15-F05-VAL-014-validate-f05.md
 */
export function writeTextStepValues(document: Document, text: GuidedText): void {
  for (const [id, value] of [
    ["text-content", text.content],
    ["text-font", text.font],
    ["text-weight", String(text.weight)],
    ["text-color", text.color],
    ["text-case", text.textCase],
    ["text-direction", text.direction],
    ["text-alignment", text.alignment],
  ] as const) {
    writeFieldValue(document, id, value);
  }

  writeCheckedValue(document, "text-underline", text.isUnderlined);
  writeCheckedValue(document, "text-strike", text.isStruckThrough);
  selectElement(document, "text-italic").setAttribute("aria-pressed", String(text.isItalic));

  for (const [id, value] of [
    ["text-size", text.size],
    ["text-opacity", text.opacity],
    ["text-letter-spacing", text.letterSpacing],
    ["text-word-spacing", text.wordSpacing],
    ["shape-x", text.x],
    ["shape-y", text.y],
    ["angle", text.angle],
  ] as const) {
    writeInputValue(document, id, value);
  }

  // The keys of `TOOL_PLACEMENT`, the placements each tool kept.
  delete selectElement(document, "placement").dataset["textPlacement"];
  delete selectElement(document, "placement").dataset["shapePlacement"];
}

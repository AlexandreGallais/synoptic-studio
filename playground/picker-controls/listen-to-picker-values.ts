import { DEGREES_PER_TURN, PERCENT_SLIDER_RANGE, clamp, toOneDecimal, toShownHue } from "../../src";
import {
  applyPickedColor,
  readPickedColor,
  showColorPicker,
  showModelChosen,
} from "../color-picker";
import { readNumber, selectElement } from "../page";

/** One of the picker's value fields and the value it sets. */
type ValueField = {
  /** The field's id. */
  readonly id: string;
  /** The value it sets. */
  readonly key: "hue" | "level" | "saturation";
};

/** The picker's value fields: the hue, the saturation and the lightness or brightness. */
const VALUE_FIELDS: readonly ValueField[] = [
  { id: "color-hue-value", key: "hue" },
  { id: "color-saturation-value", key: "saturation" },
  { id: "color-level-value", key: "level" },
];

/**
 * Reads a number typed in one of the picker's value fields as the picker holds it: a hue from 0
 * to 360, a hue of 360 shown 0.0 (`toShownHue`), a saturation, lightness or brightness from 0 to
 * 100, to one decimal (`toOneDecimal`); a number beyond an end counts at that end (`clamp`).
 *
 * @kind procedure
 * @param document - page document
 * @param field - the field's id and the value it sets
 * @returns the value; `NaN` for a field left empty
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
function readTypedValue(document: Document, field: ValueField): number {
  const typed = readNumber(document, field.id);

  return field.key === "hue"
    ? toShownHue(clamp(typed, 0, DEGREES_PER_TURN))
    : toOneDecimal(clamp(typed, 0, PERCENT_SLIDER_RANGE.maximum));
}

/**
 * Applies a number typed in one of the picker's value fields, the color following
 * (`applyPickedColor`); a field left empty shows its value again.
 *
 * @kind procedure
 * @param document - page document
 * @param field - the field's id and the value it sets
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
function applyTypedValue(document: Document, field: ValueField, update: () => void): void {
  const value = readTypedValue(document, field);

  if (Number.isNaN(value)) {
    showColorPicker(document);

    return;
  }

  applyPickedColor(document, { ...readPickedColor(document), [field.key]: value }, update);
}

/**
 * Listens to the picker's values: a number typed in the hue, the saturation or the lightness or
 * brightness sets it, the color following (`applyPickedColor`), and a field left empty shows its
 * value again; another model chosen shows the color's values in it, the hue kept
 * (`showModelChosen`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function listenToPickerValues(document: Document, update: () => void): void {
  for (const field of VALUE_FIELDS) {
    selectElement(document, field.id).addEventListener("change", () => {
      applyTypedValue(document, field, update);
    });
  }

  selectElement(document, "color-model").addEventListener("change", () => {
    showModelChosen(document);
  });
}

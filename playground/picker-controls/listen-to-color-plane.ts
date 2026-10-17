import { nearestInteger, planeValuesAfterKey, planeValuesAt } from "../../src";
import { applyPickedColor, readPickedColor } from "../color-picker";
import { selectElement } from "../page";

import { listenToDrag } from "./listen-to-drag";

import type { PlaneValues } from "../../src";

/**
 * Applies a place picked on the plane: its saturation and lightness or brightness, each a whole
 * number as the picker shows them (`nearestInteger`; ADR-0058), the hue the picker holds kept, even for a
 * grey (ADR-0056, decision 3).
 *
 * @kind procedure
 * @param document - page document
 * @param values - the saturation and the lightness or brightness picked
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
function applyPlaneValues(document: Document, values: PlaneValues, update: () => void): void {
  applyPickedColor(
    document,
    {
      ...readPickedColor(document),
      level: nearestInteger(values.level),
      saturation: nearestInteger(values.saturation),
    },
    update,
  );
}

/**
 * Listens to the plane: a click or a drag picks the saturation across and the lightness or
 * brightness up (`planeValuesAt`); the arrows, Page and Home keys move them
 * (`planeValuesAfterKey`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function listenToColorPlane(document: Document, update: () => void): void {
  const plane = selectElement(document, "color-plane");

  listenToDrag(plane, ({ point, size }) => {
    applyPlaneValues(document, planeValuesAt(point, size), update);
  });

  plane.addEventListener("keydown", (event) => {
    const values = planeValuesAfterKey(event.key, readPickedColor(document));

    if (values === undefined) {
      return;
    }

    event.preventDefault();
    applyPlaneValues(document, values, update);
  });
}

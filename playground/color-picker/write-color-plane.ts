import { PERCENT, pickedColorToHex, planePointOf } from "../../src";
import { selectElement } from "../page";

import type { ColorModel, PickedColor } from "../../src";

/** The plane's size in percent of itself, for its mark's place in CSS. */
const PLANE_PERCENT = { height: PERCENT, width: PERCENT };

/** The saturation and brightness of a hue at its purest, the right end of HSB's top row. */
const PUREST = { level: PERCENT, saturation: PERCENT };

/**
 * Shows the picker's plane for the hue it holds: its model, which paints it (HSB: white to the
 * hue across, black down; HSL: grey to the hue across, white up and black down), the hue at its
 * purest from the library's conversion (`pickedColorToHex`), the mark at its saturation and
 * lightness or brightness (`planePointOf`), and its name and values for a screen reader.
 *
 * @kind procedure
 * @param document - page document
 * @param picked - the values the picker holds
 * @param model - the model chosen
 * @see US-035
 */
export function writeColorPlane(document: Document, picked: PickedColor, model: ColorModel): void {
  const plane = selectElement(document, "color-plane");
  const levelName = model === "hsb" ? "brightness" : "lightness";
  const mark = planePointOf(picked, PLANE_PERCENT);
  const markStyle = selectElement(document, "color-plane-mark").style;

  plane.dataset["model"] = model;
  plane.style.setProperty("--hue-color", pickedColorToHex({ ...PUREST, hue: picked.hue }, "hsb"));
  plane.setAttribute("aria-label", `Saturation and ${levelName}`);
  plane.setAttribute("aria-valuenow", String(picked.saturation));

  plane.setAttribute(
    "aria-valuetext",
    `Saturation ${String(picked.saturation)} %, ${levelName} ${String(picked.level)} %`,
  );

  markStyle.left = `${String(mark.x)}%`;
  markStyle.top = `${String(mark.y)}%`;
}

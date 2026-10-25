import { closeColorPicker } from "./close-color-picker";
import { openColorPicker } from "./open-color-picker";
import { readOpenSwatch } from "./read-open-swatch";

/**
 * Opens a swatch's picker on a click, as Figma's opens on a click on the fill swatch
 * (`REF-FIGMA-COLOR-PICKER`), or closes it on a second click (`RULE-interaction-colors`,
 * provisional).
 *
 * @kind procedure
 * @param document - page document
 * @param swatch - the swatch clicked
 */
export function toggleColorPicker(document: Document, swatch: HTMLElement): void {
  if (readOpenSwatch(document) === swatch) {
    closeColorPicker(document);

    return;
  }

  openColorPicker(document, swatch);
}

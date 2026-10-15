import { closeColorPicker } from "./close-color-picker";
import { openColorPicker } from "./open-color-picker";
import { readOpenSwatch } from "./read-open-swatch";

/**
 * Opens a swatch's picker on a click, or closes it on a second click (`REF-FIGMA-COLOR-PICKER`;
 * `docs/domain/interaction.md` §5).
 *
 * @kind procedure
 * @param document - page document
 * @param swatch - the swatch clicked
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function toggleColorPicker(document: Document, swatch: HTMLElement): void {
  if (readOpenSwatch(document) === swatch) {
    closeColorPicker(document);

    return;
  }

  openColorPicker(document, swatch);
}

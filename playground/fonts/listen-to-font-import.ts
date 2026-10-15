import { runTask, selectElement, writeText } from "../page";

import { loadFontFiles } from "./load-font-files";

/**
 * Imports the files chosen (`loadFontFiles`), says what was read or refused, then redraws.
 *
 * @kind procedure
 * @param document - page document
 * @param files - the files chosen
 * @param update - redraws the playground
 * @returns once the page says it
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
async function showFontFiles(
  document: Document,
  files: readonly File[],
  update: () => void,
): Promise<void> {
  writeText(document, "font-message", await loadFontFiles(document, files, update));
  update();
}

/**
 * Listens to the font file picker (`accept=".ttf,.otf,.ttc"`, `REF-MDN-INPUT-FILE`): the files chosen
 * are imported, each file read or refused on its own, the page says which and why, then redraws
 * (`showFontFiles`); the picker is emptied, so that the same file can be chosen
 * again.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function listenToFontImport(document: Document, update: () => void): void {
  const picker = selectElement(document, "font-file");

  picker.addEventListener("change", () => {
    if (!(picker instanceof HTMLInputElement)) {
      return;
    }

    const files = [...(picker.files ?? [])];

    picker.value = "";

    runTask(
      document,
      async () => {
        await showFontFiles(document, files, update);
      },
      update,
    );
  });
}

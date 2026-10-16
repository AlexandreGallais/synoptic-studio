import { waitForNextTask } from "../page";

import { readInstalledFace } from "./read-installed-face";

import type { InstalledFaceRead } from "./installed-face-read";
import type { InstalledFont } from "../fonts";

/**
 * Reads the faces of a family one after the other (`readInstalledFace`), the page answering before
 * each file is read (`waitForNextTask`), so that a family of large files never freezes it.
 *
 * @kind procedure
 * @param view - the page's window
 * @param fonts - the faces, as the browser lists them
 * @returns their reads, in the same order
 * @see REF-WEBDEV-LONG-TASKS
 */
export async function readFacesInTurn(
  view: Window,
  fonts: readonly InstalledFont[],
): Promise<readonly InstalledFaceRead[]> {
  const [font, ...rest] = fonts;

  if (font === undefined) {
    return [];
  }

  await waitForNextTask(view);

  const read = await readInstalledFace(font);

  return [read, ...(await readFacesInTurn(view, rest))];
}

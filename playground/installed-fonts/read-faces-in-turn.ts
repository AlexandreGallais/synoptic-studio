import { waitForNextTask } from "../page";

import { readInstalledFace } from "./read-installed-face";

import type { InstalledFaceRead } from "./installed-face-read";
import type { InstalledFont } from "../fonts";

/**
 * Reads a face's read without its file, its metrics alone kept, so that its bytes are released
 * once it is checked (US-062).
 *
 * @kind procedure
 * @param read - the read of a face, its file held
 * @returns the same read, its face holding no file; a refusal as it is
 * @see RULE-shapes-text
 */
function readWithoutFile(read: InstalledFaceRead): InstalledFaceRead {
  return read.face === undefined
    ? read
    : { ...read, face: { ...read.face, face: { ...read.face.face, file: undefined } } };
}

/**
 * Reads the faces of a family ahead, one after the other: each face's file read whole and checked
 * (`readInstalledFace`), then its bytes released at once, its metrics alone kept
 * (`readWithoutFile`), so that the memory held stays that of one file at a time, a file of 192 MB
 * among them (US-062); the page answers before each file is read (`waitForNextTask`), so that a
 * family of large files never freezes it.
 *
 * @kind procedure
 * @param view - the page's window
 * @param fonts - the faces, as the browser lists them
 * @returns their reads, in the same order, no face holding its file
 * @see REF-WEBDEV-LONG-TASKS
 */
export async function readFacesInTurn(
  view: Pick<Window, "setTimeout">,
  fonts: readonly InstalledFont[],
): Promise<readonly InstalledFaceRead[]> {
  const [font, ...rest] = fonts;

  if (font === undefined) {
    return [];
  }

  await waitForNextTask(view);

  const read = readWithoutFile(await readInstalledFace(font));

  return [read, ...(await readFacesInTurn(view, rest))];
}

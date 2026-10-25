import { fontsWithFace } from "editor";

import { runTask, writeText } from "../page";

import { readFaceName } from "./read-face-name";
import { readFontSession } from "./read-font-session";
import { writeFontSession } from "./write-font-session";

import type { NamedFontFace } from "editor";

/**
 * Reads the style of a face as `FontFace` takes it, as an `@font-face` rule's descriptors.
 *
 * @kind procedure
 * @param face - its weight and style
 * @returns its `style` and `weight` descriptors
 */
function readFontFaceDescriptors(
  face: Pick<NamedFontFace["face"], "isItalic" | "weight">,
): FontFaceDescriptors {
  return { style: face.isItalic ? "italic" : "normal", weight: String(face.weight) };
}

/**
 * Creates a face the page can draw from its file's bytes, with the browser's `FontFace`
 * interface, its family given as its name, never as a CSS string: Chrome keeps the `family`
 * argument as it is, never parsed (`REF-CHROMIUM-FONT-FACE-CODE`), so the quotes of
 * `"IBM Plex Sans"` became part of the name, no text's `font-family` matched the face, and the
 * installed family of that name was drawn instead — IBM Plex Sans 450 drawn with its 500 face,
 * 91.922 wide where 91.264 is measured. Bytes the browser cannot parse still give a face,
 * whose status is "error" and whose `load()` rejects with a `SyntaxError` (`REF-CSS-FONT-LOADING`).
 *
 * @kind procedure
 * @param view - the page's window
 * @param named - the face read and its family
 * @returns the face; undefined without its file or in a browser without that interface
 */
function createPageFontFace(
  view: Window & typeof globalThis,
  named: NamedFontFace,
): FontFace | undefined {
  const { file } = named.face;
  const descriptors = readFontFaceDescriptors(named.face);

  return file === undefined || !("FontFace" in view)
    ? undefined
    : new view.FontFace(named.family, Uint8Array.from(file.bytes), descriptors);
}

/**
 * Adds a face to the page's fonts, then redraws once the browser has loaded it.
 *
 * @kind procedure
 * @param document - page document
 * @param face - the face; nothing is done without one
 * @param update - redraws the playground
 * @returns once the face is loaded and the page redrawn
 * @throws {DOMException} a `SyntaxError` when the browser cannot parse the face
 */
async function loadFontFace(
  document: Document,
  face: FontFace | undefined,
  update: () => void,
): Promise<void> {
  if (face === undefined) {
    return;
  }

  document.fonts.add(face);
  await face.load();
  update();
}

/**
 * Says that the browser could not load a face, in one line naming it and giving its reason, then
 * redraws: the text stays measured with the face, and is drawn with another one.
 *
 * @kind procedure
 * @param document - page document
 * @param named - the face read and its family
 * @param error - the browser's reason
 */
function writeLoadFailure(document: Document, named: NamedFontFace, error: unknown): void {
  writeText(
    document,
    "font-message",
    `${readFaceName(named.family, named.face)} could not be loaded by the browser (${String(error)}): measured with it, drawn with another face.`,
  );
}

/**
 * Lets the page draw a face read from a file with its own outlines, a font imported being
 * possibly not installed: the face added to the page's fonts (`loadFontFace`), then the page
 * redrawn once it is loaded. A browser without that interface, or that cannot parse the face,
 * draws the text with an installed font of that name, else its fallback: measured all the same;
 * a failed load is said (`writeLoadFailure`), never left unseen.
 *
 * @kind procedure
 * @param document - page document
 * @param named - the face read and its family
 * @param update - redraws the playground
 */
function registerFontFace(document: Document, named: NamedFontFace, update: () => void): void {
  const view = document.defaultView;

  if (view === null) {
    return;
  }

  runTask(
    document,
    async () => {
      await loadFontFace(document, createPageFontFace(view, named), update);
    },
    (error) => {
      writeLoadFailure(document, named, error);
      update();
    },
  );
}

/**
 * Adds a face read from its file to the session's families (`fontsWithFace`), its file held,
 * and lets the page draw it with its own outlines (`registerFontFace`).
 *
 * @kind procedure
 * @param document - page document
 * @param named - the face read, its file held, and its family
 * @param update - redraws the playground once the page can draw the face
 */
export function addFontFace(document: Document, named: NamedFontFace, update: () => void): void {
  const session = readFontSession(document);

  writeFontSession(document, { ...session, kept: fontsWithFace(session.kept, named) });
  registerFontFace(document, named, update);
}

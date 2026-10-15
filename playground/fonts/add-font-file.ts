import { fontsWithFace, formatFontFamilyName, parseFontFace } from "../../src";
import { runTask } from "../page";

import { readFontSession } from "./read-font-session";
import { writeFontSession } from "./write-font-session";

import type { NamedFontFace } from "../../src";

/**
 * Reads the style of a face as `FontFace` takes it, as an `@font-face` rule's descriptors.
 *
 * @kind procedure
 * @param face - its weight and style
 * @returns its `style` and `weight` descriptors
 * @see REF-CSS-FONT-LOADING
 */
function readFontFaceDescriptors(
  face: Pick<NamedFontFace["face"], "isItalic" | "weight">,
): FontFaceDescriptors {
  return { style: face.isItalic ? "italic" : "normal", weight: String(face.weight) };
}

/**
 * Creates a face the page can draw from its file's bytes, with the browser's `FontFace`
 * interface, its family written as a CSS string (`formatFontFamilyName`), as an `@font-face`
 * rule's `font-family` is parsed. A family or bytes the browser cannot parse still give a face,
 * whose status is "error" and whose `load()` rejects with a `SyntaxError` (`REF-CSS-FONT-LOADING`).
 *
 * @kind procedure
 * @param view - the page's window
 * @param named - the face read and its family
 * @returns the face; undefined without its file or in a browser without that interface
 * @see REF-CSS-FONT-LOADING
 */
function createPageFontFace(
  view: Window & typeof globalThis,
  named: NamedFontFace,
): FontFace | undefined {
  const { file } = named.face;
  const family = formatFontFamilyName(named.family);
  const descriptors = readFontFaceDescriptors(named.face);

  return file === undefined || !("FontFace" in view)
    ? undefined
    : new view.FontFace(family, Uint8Array.from(file.bytes), descriptors);
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
 * @see REF-CSS-FONT-LOADING
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
 * Lets the page draw a face read from a file with its own outlines, a font imported being
 * possibly not installed: the face added to the page's fonts (`loadFontFace`), then the page
 * redrawn once it is loaded. A browser without that interface, or that cannot parse the face,
 * draws the text with an installed font of that name, else its fallback: measured all the same.
 *
 * @kind procedure
 * @param document - page document
 * @param named - the face read and its family
 * @param update - redraws the playground
 * @see REF-CSS-FONT-LOADING
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
    update,
  );
}

/**
 * Reads a font file's bytes into a face (`parseFontFace`) and adds it to the session's families
 * (`fontsWithFace`), the page then able to draw it.
 *
 * @kind procedure
 * @param document - page document
 * @param bytes - the bytes of a font file
 * @param update - redraws the playground once the page can draw the face
 * @returns the face added and its family; undefined for a file that is not a TrueType or OpenType font
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function addFontFile(
  document: Document,
  bytes: Readonly<Uint8Array>,
  update: () => void,
): NamedFontFace | undefined {
  const named = parseFontFace(bytes);

  if (named === undefined) {
    return undefined;
  }

  const session = readFontSession(document);

  writeFontSession(document, { ...session, kept: fontsWithFace(session.kept, named) });
  registerFontFace(document, named, update);

  return named;
}

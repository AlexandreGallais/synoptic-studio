import { parseNamedFaceParts } from "../../src";
import { addFontFile, readJoinedFile } from "../fonts";

import type { InstalledFaceRead } from "./installed-face-read";
import type { InstalledFont } from "../fonts";

/** Why a file read is refused: it is not a TrueType or OpenType font, nor a collection of them. */
const NOT_A_FONT = "not a TrueType or OpenType font";

/** An installed face and the bytes the browser gave for it. */
type GivenFace = {
  /** The face, as the browser lists it. */
  readonly font: InstalledFont;
  /** The bytes of its file, or of its whole collection. */
  readonly bytes: Uint8Array<ArrayBuffer>;
};

/**
 * Reads the bytes of an installed face, as the browser gives them (`blob()`,
 * `REF-WICG-LOCAL-FONT-ACCESS`). Chrome rejects with a `TypeError` when it cannot find the face by
 * its PostScript name or Skia gives no stream (`REF-CHROMIUM-FONT-BLOB-CODE`): the page has no
 * other way to read them, so the refusal names the face, gives the browser's reason and proposes
 * to import its file.
 *
 * @kind procedure
 * @param font - a face the browser lists
 * @returns its bytes; the refusal, in one line, when the browser cannot give them
 * @see REF-CHROMIUM-FONT-BLOB-CODE
 */
async function readFaceBytes(font: InstalledFont): Promise<Uint8Array<ArrayBuffer> | string> {
  try {
    const blob = await font.blob();

    return new Uint8Array(await blob.arrayBuffer());
  } catch (error: unknown) {
    return error instanceof TypeError
      ? `${font.fullName}: ${error.message} Import its file with Import a font`
      : `${font.fullName}: ${String(error)}`;
  }
}

/**
 * Reads the file of an installed face from the bytes the browser gave for it
 * (`parseNamedFaceParts`): a face alone is its own file; in a whole collection, as a browser may
 * give it, the face of its PostScript name (name ID 6, as `FontData.postscriptName`) is found by
 * reading each face's name in the collection, and only that face is joined from its parts
 * (`readJoinedFile`), the other faces never written (US-052).
 *
 * @kind procedure
 * @param font - a face the browser lists
 * @param bytes - the bytes the browser gave for it
 * @returns its file; undefined when a collection holds no face of that name
 * @throws {DOMException} when the browser cannot hold the joined file
 * @see REF-OT-OTFF
 */
async function readFaceFile(
  font: InstalledFont,
  bytes: Uint8Array<ArrayBuffer>,
): Promise<Uint8Array<ArrayBuffer> | undefined> {
  const parts = parseNamedFaceParts(bytes, font.postscriptName);

  return parts === undefined ? undefined : await readJoinedFile(parts);
}

/**
 * Reads the file of one face into a face added to the session's families (`addFontFile`), the
 * page given that face alone, as a font file of its own.
 *
 * @kind procedure
 * @param document - page document
 * @param file - the bytes of one face, a font file of its own
 * @param update - redraws the playground
 * @returns the face read; or why it was refused, a file that is not a font
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readFaceFromFile(
  document: Document,
  file: Readonly<Uint8Array>,
  update: () => void,
): InstalledFaceRead {
  const face = addFontFile(document, file, update);

  return { face, refusal: face === undefined ? NOT_A_FONT : "" };
}

/**
 * Reads an installed face from the bytes the browser gave for it: its own file, or its face in
 * a whole collection (`readFaceFile`).
 *
 * @kind procedure
 * @param document - page document
 * @param given - the face and the bytes the browser gave for it
 * @param update - redraws the playground
 * @returns the face read; or why it was refused
 * @throws {DOMException} when the browser cannot hold the joined file
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
async function readFaceFromBytes(
  document: Document,
  given: GivenFace,
  update: () => void,
): Promise<InstalledFaceRead> {
  const file = await readFaceFile(given.font, given.bytes);

  return file === undefined
    ? { face: undefined, refusal: `no face named ${given.font.postscriptName} in its collection` }
    : readFaceFromFile(document, file, update);
}

/**
 * Reads one installed face from the bytes the browser gives for it, or says why the browser
 * could not give them.
 *
 * @kind procedure
 * @param document - page document
 * @param font - a face the browser lists
 * @param update - redraws the playground
 * @returns the face read; or why it was refused
 * @throws {DOMException} when the browser cannot hold the joined file
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
async function readFaceRead(
  document: Document,
  font: InstalledFont,
  update: () => void,
): Promise<InstalledFaceRead> {
  const bytes = await readFaceBytes(font);

  return typeof bytes === "string"
    ? { face: undefined, refusal: bytes }
    : await readFaceFromBytes(document, { bytes, font }, update);
}

/**
 * Reads one installed face, as the browser gives its file (`blob()`,
 * `REF-WICG-LOCAL-FONT-ACCESS`), into a face added to the session's families; a face whose file
 * the browser cannot give, or that is not a font, is refused, said why in one line, and no error
 * is left uncaught: one thrown while its file is read, joined or added — a file the browser
 * cannot hold among them — refuses the face, named, with the error (US-052).
 *
 * @kind procedure
 * @param document - page document
 * @param font - a face the browser lists
 * @param update - redraws the playground
 * @returns the face read; or why it was refused
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
export async function readInstalledFace(
  document: Document,
  font: InstalledFont,
  update: () => void,
): Promise<InstalledFaceRead> {
  try {
    return await readFaceRead(document, font, update);
  } catch (error: unknown) {
    return { face: undefined, refusal: `${font.fullName}: ${String(error)}` };
  }
}

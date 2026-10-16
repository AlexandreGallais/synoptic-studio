import { parseFontFace, parseNamedFaceParts, unlistedFaceReason } from "../../src";
import { UNLISTED_FACE_REASONS, readJoinedFile } from "../fonts";

import type { InstalledFaceRead } from "./installed-face-read";
import type { NamedFontFace } from "../../src";
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
 * Reads what a face read from its file gives (US-056): the face, kept, when the editor can draw,
 * measure and embed it; else left out, said why (`unlistedFaceReason`): its `fsType` forbids
 * embedding it, or it lacks a letter of the ISO basic Latin alphabet.
 *
 * @kind procedure
 * @param font - the face, as the browser lists it
 * @param named - the face read from its file
 * @returns the face read; or why it was left out
 * @see docs/domain/shapes.md#text
 */
function readUsableFace(font: InstalledFont, named: NamedFontFace): InstalledFaceRead {
  const leftOut = unlistedFaceReason(named.face);

  return leftOut === undefined
    ? { face: named, leftOut, refusal: "" }
    : { face: undefined, leftOut, refusal: `${font.fullName}: ${UNLISTED_FACE_REASONS[leftOut]}` };
}

/**
 * Reads the file of one face into a face (`parseFontFace`), the session left as it is, kept only
 * when it can be listed (`readUsableFace`).
 *
 * @kind procedure
 * @param font - the face, as the browser lists it
 * @param file - the bytes of one face, a font file of its own
 * @returns the face read; or why it was refused, a file that is not a font or a face left out
 * @see REF-OT-OTFF
 */
function readFaceFromFile(font: InstalledFont, file: Readonly<Uint8Array>): InstalledFaceRead {
  const named = parseFontFace(file);

  return named === undefined
    ? { face: undefined, leftOut: "unreadable", refusal: NOT_A_FONT }
    : readUsableFace(font, named);
}

/**
 * Reads an installed face from the bytes the browser gave for it: its own file, or its face in
 * a whole collection (`readFaceFile`).
 *
 * @kind procedure
 * @param given - the face and the bytes the browser gave for it
 * @returns the face read; or why it was refused
 * @throws {DOMException} when the browser cannot hold the joined file
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
async function readFaceFromBytes(given: GivenFace): Promise<InstalledFaceRead> {
  const file = await readFaceFile(given.font, given.bytes);

  return file === undefined
    ? {
        face: undefined,
        leftOut: "unreadable",
        refusal: `no face named ${given.font.postscriptName} in its collection`,
      }
    : readFaceFromFile(given.font, file);
}

/**
 * Reads one installed face from the bytes the browser gives for it, or says why the browser
 * could not give them.
 *
 * @kind procedure
 * @param font - a face the browser lists
 * @returns the face read; or why it was refused
 * @throws {DOMException} when the browser cannot hold the joined file
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
async function readFaceRead(font: InstalledFont): Promise<InstalledFaceRead> {
  const bytes = await readFaceBytes(font);

  return typeof bytes === "string"
    ? { face: undefined, leftOut: "unreadable", refusal: bytes }
    : await readFaceFromBytes({ bytes, font });
}

/**
 * Reads one installed face, as the browser gives its file (`blob()`,
 * `REF-WICG-LOCAL-FONT-ACCESS`), into a face, the session left as it is; a face whose file
 * the browser cannot give, or that is not a font, is refused, said why in one line, and no error
 * is left uncaught: one thrown while its file is read, joined or added — a file the browser
 * cannot hold among them — refuses the face, named, with the error (US-052). A face whose
 * `fsType` forbids embedding it, or that lacks a letter of the ISO basic Latin alphabet, is left
 * out too, said why (US-056). The caller keeps the
 * face, with its file when its family is chosen, its metrics alone when read ahead (US-053).
 *
 * @kind procedure
 * @param font - a face the browser lists
 * @returns the face read; or why it was refused
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
export async function readInstalledFace(font: InstalledFont): Promise<InstalledFaceRead> {
  try {
    return await readFaceRead(font);
  } catch (error: unknown) {
    return {
      face: undefined,
      leftOut: "unreadable",
      refusal: `${font.fullName}: ${String(error)}`,
    };
  }
}

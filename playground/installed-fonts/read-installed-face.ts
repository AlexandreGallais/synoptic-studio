import { parseCollectionFaces, parsePostScriptName } from "../../src";
import { addFontFile } from "../fonts";

import type { InstalledFaceRead } from "./installed-face-read";
import type { InstalledFont } from "../fonts";

/** Why a file read is refused: it is not a TrueType or OpenType font, nor a collection of them. */
const NOT_A_FONT = "not a TrueType or OpenType font";

/** An installed face and the bytes the browser gave for it. */
type GivenFace = {
  /** The face, as the browser lists it. */
  readonly font: InstalledFont;
  /** The bytes of its file, or of its whole collection. */
  readonly bytes: Uint8Array;
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
async function readFaceBytes(font: InstalledFont): Promise<Uint8Array | string> {
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
 * Finds the file of an installed face among those its bytes hold (`parseCollectionFaces`): a face
 * alone is its own file; a whole collection, as a browser may give it, is searched for the face of
 * its PostScript name (name ID 6, as `FontData.postscriptName`), the other faces left out.
 *
 * @kind procedure
 * @param font - a face the browser lists
 * @param bytes - the bytes the browser gave for it
 * @returns its file; undefined when a collection holds no face of that name
 * @see REF-OT-OTFF
 */
function readFaceFile(font: InstalledFont, bytes: Uint8Array): Readonly<Uint8Array> | undefined {
  const files = parseCollectionFaces(bytes);
  const [only] = files;

  return files.length === 1
    ? only
    : files.find((file) => parsePostScriptName(file) === font.postscriptName);
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
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readFaceFromBytes(
  document: Document,
  given: GivenFace,
  update: () => void,
): InstalledFaceRead {
  const file = readFaceFile(given.font, given.bytes);

  return file === undefined
    ? { face: undefined, refusal: `no face named ${given.font.postscriptName} in its collection` }
    : readFaceFromFile(document, file, update);
}

/**
 * Reads one installed face, as the browser gives its file (`blob()`,
 * `REF-WICG-LOCAL-FONT-ACCESS`), into a face added to the session's families; a face whose file
 * the browser cannot give, or that is not a font, is refused, said why in one line, and no error
 * is left uncaught.
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
  const bytes = await readFaceBytes(font);

  return typeof bytes === "string"
    ? { face: undefined, refusal: bytes }
    : readFaceFromBytes(document, { bytes, font }, update);
}

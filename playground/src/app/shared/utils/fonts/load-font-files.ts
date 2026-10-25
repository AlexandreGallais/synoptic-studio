import { facePartsToFile, parseCollectionFaces, parseFontFace, unlistedFaceReason } from "editor";

import { addFontFace } from "./add-font-face";
import { readFaceName } from "./read-face-name";
import { UNLISTED_FACE_REASONS } from "./unlisted-face-reasons";
import { writeFontOptions } from "./write-font-options";

import type { NamedFontFace } from "editor";

/** What the page says of one file chosen, and the family it added. */
type LoadedFile = {
  /** The line said. */
  readonly line: string;
  /** The family of the face kept; empty for a file refused. */
  readonly family: string;
};

/**
 * Reads the faces of a font file: its face, or each face of a collection
 * (`parseCollectionFaces`), each joined from its parts into one buffer (`facePartsToFile`) and
 * read alone (`parseFontFace`).
 *
 * @kind procedure
 * @param bytes - the bytes of a font file or of a collection
 * @returns the faces read, in the file's order; none for a file that is not a font
 * @throws {RangeError} when the engine cannot allocate a face's joined file
 */
function readFileFaces(bytes: Uint8Array<ArrayBuffer>): readonly NamedFontFace[] {
  return parseCollectionFaces(bytes).flatMap((parts) => {
    const named = parseFontFace(facePartsToFile(parts));

    return named === undefined ? [] : [named];
  });
}

/**
 * Reads what the page says of the faces of a file refused because the editor cannot list them
 * (ADR-0062): each named, with its reason (`unlistedFaceReason`).
 *
 * @kind procedure
 * @param file - the file chosen
 * @param faces - the faces read from it
 * @returns one sentence per face refused; none when every face is kept
 */
function readRefusedFaces(file: File, faces: readonly NamedFontFace[]): readonly string[] {
  return faces.flatMap((named) => {
    const reason = unlistedFaceReason(named.face);
    const said = reason === undefined ? undefined : UNLISTED_FACE_REASONS[reason];

    return said === undefined
      ? []
      : `${file.name}: ${readFaceName(named.family, named.face)} refused, ${said}.`;
  });
}

/**
 * Reads what the page says of a file whose faces are read: the faces kept, imported, then those
 * refused (`readRefusedFaces`); a file that is not a font, refused.
 *
 * @kind procedure
 * @param file - the file chosen
 * @param faces - the faces read from it
 * @param kept - those kept
 * @returns the line said
 */
function readImportLine(
  file: File,
  faces: readonly NamedFontFace[],
  kept: readonly NamedFontFace[],
): string {
  const names = kept.map((named) => readFaceName(named.family, named.face));
  const imported = names.length === 0 ? [] : [`${names.join(", ")} imported.`];

  return faces.length === 0
    ? `${file.name} is not a TrueType or OpenType font file (.ttf, .otf, .ttc): refused.`
    : [...imported, ...readRefusedFaces(file, faces)].join(" ");
}

/**
 * Reads one file chosen and keeps its faces, saying what was read or why it was refused: a font
 * file gives its face, a collection (`.ttc`) each of its faces (`readFileFaces`), each given to
 * the page alone (`addFontFace`); a face lacking a letter of the basic Latin alphabet, or whose
 * `fsType` forbids embedding it, is refused, as an installed one is (ADR-0062).
 *
 * @kind procedure
 * @param document - page document
 * @param file - a file the designer chose
 * @param update - redraws the playground
 * @returns the line said, and the family of the last face kept, empty when none is kept
 * @throws {DOMException} when the browser cannot read the file's bytes
 * @throws {RangeError} when the engine cannot allocate a face's file
 */
async function loadFontFile(
  document: Document,
  file: File,
  update: () => void,
): Promise<LoadedFile> {
  const faces = readFileFaces(new Uint8Array(await file.arrayBuffer()));
  const kept = faces.filter((named) => unlistedFaceReason(named.face) === undefined);

  for (const named of kept) {
    addFontFace(document, named, update);
  }

  return { family: kept.at(-1)?.family ?? "", line: readImportLine(file, faces, kept) };
}

/**
 * Reads what the page says of a file once its read is settled: what it read, or why the browser
 * could not read it.
 *
 * @kind procedure
 * @param file - the file chosen
 * @param settled - its read, fulfilled or rejected
 * @returns the line said, and the family kept, empty for a file refused
 */
function readLoadedFile(file: File, settled: PromiseSettledResult<LoadedFile>): LoadedFile {
  return settled.status === "fulfilled"
    ? settled.value
    : { family: "", line: `${file.name} could not be read (${String(settled.reason)}).` };
}

/**
 * Imports the font files the designer chose (`REF-MDN-INPUT-FILE`): each read once from its bytes
 * (`REF-W3C-FILE-API`) into a face kept for the session, its family added to the font
 * list and the last family read chosen, so that the text is measured with it; a file that is not
 * a font, or that the browser could not read, is refused, said why, the others kept; a face the
 * editor cannot list, as an installed one, is refused, said why.
 *
 * @kind procedure
 * @param document - page document
 * @param files - the files chosen
 * @param update - redraws the playground
 * @returns one line per file, what was read or why it was refused
 */
export async function loadFontFiles(
  document: Document,
  files: readonly File[],
  update: () => void,
): Promise<string> {
  const settled = await Promise.allSettled(
    files.map(async (file) => await loadFontFile(document, file, update)),
  );
  const read = files.map((file, index) =>
    readLoadedFile(file, settled[index] ?? { reason: "", status: "rejected" }),
  );
  const family = read.findLast((file) => file.family !== "")?.family;

  if (family !== undefined) {
    writeFontOptions(document, family);
  }

  return read.map((file) => file.line).join(" ");
}

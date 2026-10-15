import { parseCollectionFaces } from "../../src";

import { addFontFile } from "./add-font-file";
import { readFaceName } from "./read-face-name";
import { writeFontOptions } from "./write-font-options";

import type { NamedFontFace } from "../../src";

/** What the page says of one file chosen, and the family it added. */
type LoadedFile = {
  /** The line said. */
  readonly line: string;
  /** The family of the face kept; empty for a file refused. */
  readonly family: string;
};

/**
 * Adds the faces of a font file to the session's families (`addFontFile`): its face, or each face
 * of a collection (`parseCollectionFaces`), each given to the page alone.
 *
 * @kind procedure
 * @param document - page document
 * @param bytes - the bytes of a font file or of a collection
 * @param update - redraws the playground
 * @returns the faces added, in the file's order; none for a file that is not a font
 * @see REF-OT-OTFF
 */
function addFontFaces(
  document: Document,
  bytes: Uint8Array,
  update: () => void,
): readonly NamedFontFace[] {
  return parseCollectionFaces(bytes).flatMap((face) => {
    const named = addFontFile(document, face, update);

    return named === undefined ? [] : [named];
  });
}

/**
 * Reads one file chosen and keeps its faces, saying what was read or why it was refused: a font
 * file gives its face, a collection (`.ttc`) each of its faces (`parseCollectionFaces`), each
 * given to the page alone.
 *
 * @kind procedure
 * @param document - page document
 * @param file - a file the designer chose
 * @param update - redraws the playground
 * @returns the line said, and the family of the last face kept, empty for a file refused
 * @throws {DOMException} when the browser cannot read the file's bytes
 * @see REF-W3C-FILE-API
 */
async function loadFontFile(
  document: Document,
  file: File,
  update: () => void,
): Promise<LoadedFile> {
  const faces = addFontFaces(document, new Uint8Array(await file.arrayBuffer()), update);
  const names = faces.map((named) => readFaceName(named.family, named.face));

  return {
    family: faces.at(-1)?.family ?? "",
    line:
      names.length === 0
        ? `${file.name} is not a TrueType or OpenType font file (.ttf, .otf, .ttc): refused.`
        : `${names.join(", ")} imported.`,
  };
}

/**
 * Reads what the page says of a file once its read is settled: what it read, or why the browser
 * could not read it.
 *
 * @kind procedure
 * @param file - the file chosen
 * @param settled - its read, fulfilled or rejected
 * @returns the line said, and the family kept, empty for a file refused
 * @see REF-W3C-FILE-API
 */
function readLoadedFile(file: File, settled: PromiseSettledResult<LoadedFile>): LoadedFile {
  return settled.status === "fulfilled"
    ? settled.value
    : { family: "", line: `${file.name} could not be read (${String(settled.reason)}).` };
}

/**
 * Imports the font files the designer chose (`REF-MDN-INPUT-FILE`): each read once from its bytes
 * (`REF-W3C-FILE-API`) into a face kept for the session (EN-039), its family added to the font
 * list and the last family read chosen, so that the text is measured with it; a file that is not
 * a font, or that the browser could not read, is refused, said why, the others kept.
 *
 * @kind procedure
 * @param document - page document
 * @param files - the files chosen
 * @param update - redraws the playground
 * @returns one line per file, what was read or why it was refused
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
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

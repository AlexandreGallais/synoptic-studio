import { addFontFile } from "./add-font-file";
import { readFaceName } from "./read-face-name";
import { writeFontOptions } from "./write-font-options";

/**
 * Reads one file chosen and keeps its face, saying what was read or why it was refused.
 *
 * @kind procedure
 * @param document - page document
 * @param file - a file the designer chose
 * @param update - redraws the playground
 * @returns the line said, and the family kept, empty for a file refused
 * @see REF-W3C-FILE-API
 */
async function loadFontFile(
  document: Document,
  file: File,
  update: () => void,
): Promise<{ line: string; family: string }> {
  const named = addFontFile(document, new Uint8Array(await file.arrayBuffer()), update);

  return named === undefined
    ? {
        family: "",
        line: `${file.name} is not a TrueType or OpenType font file (.ttf, .otf): refused.`,
      }
    : { family: named.family, line: `${readFaceName(named.family, named.face)} imported.` };
}

/**
 * Imports the font files the designer chose (`REF-MDN-INPUT-FILE`): each read once from its bytes
 * (`REF-W3C-FILE-API`) into a face kept for the session (EN-039), its family added to the font
 * list and the last family read chosen, so that the text is measured with it; a file that is not
 * a font is refused, said why.
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
  const read = await Promise.all(
    files.map(async (file) => await loadFontFile(document, file, update)),
  );
  const family = read.findLast((file) => file.family !== "")?.family;

  if (family !== undefined) {
    writeFontOptions(document, family);
  }

  return read.map((file) => file.line).join(" ");
}

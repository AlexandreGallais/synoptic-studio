import { collectionFaceToParts, parseCollectionOffsets } from "../font-collection";

import { parsePostScriptName } from "./parse-post-script-name";

/**
 * Finds the face of a PostScript name (name ID 6, as a browser gives an installed face,
 * `FontData.postscriptName`) in the bytes given for it, as the parts of its own file: a font file
 * is its own face, one part; a collection of one face gives that face; a collection of several
 * has each face's name read in the collection itself, at its directory, and gives the parts of
 * the face of that name alone (`collectionFaceToParts`), the other faces never written (US-052).
 *
 * @kind format
 * @template Buffer - the buffer the bytes sit in, kept by every view of them
 * @param file - the bytes of a font file, or of a collection
 * @param postScriptName - the PostScript name of the face looked for
 * @returns the face's parts, joined in their order into its file; undefined when a collection of
 * several faces holds none of that name
 * @see REF-OT-OTFF
 */
export function parseNamedFaceParts<Buffer extends ArrayBufferLike>(
  file: Readonly<Uint8Array<Buffer>>,
  postScriptName: string,
): readonly Readonly<Uint8Array<ArrayBuffer | Buffer>>[] | undefined {
  const offsets = parseCollectionOffsets(
    new DataView(file.buffer, file.byteOffset, file.byteLength),
  );
  const [only] = offsets;
  const directory =
    offsets.length === 1
      ? only
      : offsets.find((offset) => parsePostScriptName(file, offset) === postScriptName);

  if (offsets.length === 0) {
    return [file];
  }

  return directory === undefined ? undefined : collectionFaceToParts(file, directory);
}

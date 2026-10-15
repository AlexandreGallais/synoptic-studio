import { collectionFaceToFile } from "./collection-face-to-file";
import { parseCollectionOffsets } from "./parse-collection-offsets";

/**
 * Reads the faces a font file holds, each as a font file of its own: a collection (`.ttc`, its
 * header `ttcf`) gives each of its faces written alone (`collectionFaceToFile`), in the order of
 * its `tableDirectoryOffsets`; any other file is given back as it is, one face or none.
 *
 * @kind format
 * @param file - the bytes of a font file or of a collection
 * @returns the file of each face; the file itself when it is not a collection
 * @see REF-OT-OTFF
 */
export function parseCollectionFaces(file: Readonly<Uint8Array>): readonly Readonly<Uint8Array>[] {
  const offsets = parseCollectionOffsets(
    new DataView(file.buffer, file.byteOffset, file.byteLength),
  );

  return offsets.length === 0
    ? [file]
    : offsets.map((directory) => collectionFaceToFile(file, directory));
}

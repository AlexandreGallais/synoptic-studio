import { collectionFaceToParts } from "./collection-face-to-parts";
import { parseCollectionOffsets } from "./parse-collection-offsets";

/**
 * Reads the faces a font file holds, each as the parts of a font file of its own: a collection
 * (`.ttc`, its header `ttcf`) gives each of its faces' parts (`collectionFaceToParts`), views of
 * its bytes, in the order of its `tableDirectoryOffsets`; any other file is given back as it is,
 * one face of one part, or none.
 *
 * @kind format
 * @template Buffer - the buffer the bytes sit in, kept by every view of them
 * @param file - the bytes of a font file or of a collection
 * @returns the parts of each face, joined in their order into its file; the file itself when it
 * is not a collection
 */
export function parseCollectionFaces<Buffer extends ArrayBufferLike>(
  file: Readonly<Uint8Array<Buffer>>,
): readonly (readonly Readonly<Uint8Array<ArrayBuffer | Buffer>>[])[] {
  const offsets = parseCollectionOffsets(
    new DataView(file.buffer, file.byteOffset, file.byteLength),
  );

  return offsets.length === 0
    ? [[file]]
    : offsets.map((directory) => collectionFaceToParts(file, directory));
}

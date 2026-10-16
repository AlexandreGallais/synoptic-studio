/**
 * Reads the file of a face from its parts (`collectionFaceToParts`), joined in their order by the
 * browser (`Blob`, `REF-W3C-FILE-API`), so that a face of a collection holding a table of 191 MB
 * — Apple Color Emoji's `sbix` — is written once, into one buffer, never spread into an array of
 * numbers past the longest one an engine allows (US-052). Only the face kept is joined.
 *
 * @kind procedure
 * @param parts - the face's parts, its directory first
 * @returns the face's file
 * @throws {DOMException} when the browser cannot hold the joined file
 * @see REF-W3C-FILE-API
 */
export async function readJoinedFile(
  parts: readonly Readonly<Uint8Array<ArrayBuffer>>[],
): Promise<Uint8Array<ArrayBuffer>> {
  const blob = new Blob([...parts]);

  return new Uint8Array(await blob.arrayBuffer());
}

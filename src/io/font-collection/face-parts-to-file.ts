import { scan } from "../../math";

/** A part of a face placed in its file: its bytes, where they start and where the next part does. */
type PlacedPart = {
  /** The part's bytes. */
  readonly part: Readonly<Uint8Array>;
  /** Offset of the part in the face's file. */
  readonly start: number;
  /** Offset of the next part: just past this one. */
  readonly end: number;
};

/**
 * Joins the parts of a face (`collectionFaceToParts`) into its file: one buffer of their total
 * size, each part written once at its place, just after the part before it
 * (`%TypedArray%.prototype.set`, `REF-ECMA-262-TYPEDARRAY-SET`), so that a table of 191 MB is
 * copied once and never spread into an array of numbers, past the longest array an engine allows.
 * The buffer is written inside this function only, then returned: the one place in `src/` where
 * the functional lint is turned off (ADR-0066, `docs/conventions/exceptions.md`).
 *
 * Formula: the part i starts at the sum of the lengths before it (a prefix sum, `scan`).
 *
 * @kind format
 * @param parts - the face's parts, its directory first
 * @returns the face's file, in a buffer of its own
 * @see REF-ECMA-262-TYPEDARRAY-SET
 */
export function facePartsToFile(parts: readonly Readonly<Uint8Array>[]): Uint8Array<ArrayBuffer> {
  const placed = scan<PlacedPart, Readonly<Uint8Array>>(
    parts,
    (before, part) => ({ end: before.end + part.length, part, start: before.end }),
    { end: 0, part: new Uint8Array(), start: 0 },
  );
  const file = new Uint8Array(placed.at(-1)?.end ?? 0);

  /* eslint-disable functional/no-expression-statements, functional/no-loop-statements -- a buffer of the final size, written once at known places inside this function, then returned: no copy of 191 MB per part (ADR-0066) */
  for (const { part, start } of placed) {
    file.set(part, start);
  }
  /* eslint-enable functional/no-expression-statements, functional/no-loop-statements */

  return file;
}

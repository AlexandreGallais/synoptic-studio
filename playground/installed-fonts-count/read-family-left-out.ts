import type { InstalledFaceLeftOut } from "./installed-face-left-out";
import type { InstalledFaceRead } from "../installed-fonts";

/** The reasons a family is left out, in the order one of its faces' reasons is taken for the family (US-056). */
const LEFT_OUT_ORDER: readonly InstalledFaceLeftOut[] = [
  "no-latin-letters",
  "not-embeddable",
  "unreadable",
];

/**
 * Reads why a family none of whose faces is listed is left out (US-056): a face read that lacks
 * the Latin letters, else one whose `fsType` forbids embedding it, else its files unreadable.
 *
 * @kind procedure
 * @param reads - the reads of its faces, none kept
 * @returns the family's reason
 * @see docs/domain/shapes.md#text
 */
export function readFamilyLeftOut(reads: readonly InstalledFaceRead[]): InstalledFaceLeftOut {
  const reasons = new Set(reads.map((read) => read.leftOut));

  return LEFT_OUT_ORDER.find((reason) => reasons.has(reason)) ?? "unreadable";
}

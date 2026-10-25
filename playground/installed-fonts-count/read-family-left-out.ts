import { LEFT_OUT_REASONS } from "./left-out-reasons";

import type { InstalledFaceLeftOut } from "./installed-face-left-out";
import type { InstalledFaceRead } from "../installed-fonts";

/**
 * Reads why a family none of whose faces is listed is left out: the first reason one of
 * its faces has, in the one order of `LEFT_OUT_REASONS` — not embeddable, no Latin letters,
 * unreadable.
 *
 * @kind procedure
 * @param reads - the reads of its faces, none kept
 * @returns the family's reason
 */
export function readFamilyLeftOut(reads: readonly InstalledFaceRead[]): InstalledFaceLeftOut {
  const reasons = new Set(reads.map((read) => read.leftOut));
  const [reason] = LEFT_OUT_REASONS.find(([key]) => reasons.has(key)) ?? ["unreadable"];

  return reason;
}

import type { RefusedFace } from "./refused-face";
import type { InstalledFont } from "../fonts";
import type { InstalledFaceRead } from "../installed-fonts";

/**
 * Reads the faces of a family refused at the first reading, each with its reason.
 *
 * @kind procedure
 * @param fonts - the family's faces, as the browser lists them
 * @param reads - their reads, in the same order
 * @returns the faces refused, in the browser's order; none when every face was read
 */
export function readRefusedFaces(
  fonts: readonly InstalledFont[],
  reads: readonly InstalledFaceRead[],
): readonly RefusedFace[] {
  return fonts.flatMap((font, index) => {
    const reason = reads[index]?.leftOut;

    return reason === undefined ? [] : { name: font.fullName, reason };
  });
}

import { clamp } from "../../math";

/** The lightest weight of a face, Thin (`REF-OT-OS2`). */
const THINNEST = 100;

/** The heaviest weight of a face, Black (`REF-OT-OS2`). */
const BLACKEST = 900;

/**
 * The weight of a face from its file's `usWeightClass`, valid from 1 to 1000 with 100 (Thin) to
 * 900 (Black) in common use (`REF-OT-OS2`): brought into 100 to 900, the weights a face of the
 * model has, so that 50 gives 100 and 950 gives 900; a weight between the hundreds is kept.
 *
 * Formula: clamp(usWeightClass, 100, 900).
 *
 * @kind domain
 * @param weightClass - the `usWeightClass` of the file, 0 to 65 535
 * @returns the face's weight, from 100 to 900
 * @see REF-OT-OS2
 */
export function faceWeight(weightClass: number): number {
  return clamp(weightClass, THINNEST, BLACKEST);
}

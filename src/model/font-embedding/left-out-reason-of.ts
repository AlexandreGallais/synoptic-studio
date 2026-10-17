import type { LeftOutReason } from "./left-out-reason";
import type { FontFile } from "../font";

/**
 * Why a face's file is not embedded: no file held, or a file whose `fsType` does not permit
 * embedding it whole (`isEmbeddable`, `DERIV-font-embedding`, step 3); none for a file that may
 * be embedded.
 *
 * @kind domain
 * @param file - the face's file; undefined when the editor does not hold it
 * @returns the reason; undefined when the file may be embedded
 * @see DERIV-font-embedding
 */
export function leftOutReasonOf(file: FontFile | undefined): LeftOutReason | undefined {
  if (file === undefined) {
    return "no-file";
  }

  return file.isEmbeddable ? undefined : "not-permitted";
}

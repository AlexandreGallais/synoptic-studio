import { SEPARATOR } from "./separator";

/**
 * Path data of several closed sub-paths drawn as one `<path>`: each starts with its own
 * `moveto`, and "subsequent "moveto" commands … represent the start of a new subpath" (§9.3.3),
 * so the sub-paths are written one after the other. No sub-path gives an empty `d`.
 *
 * @kind format
 * @param subPaths - the path data of each sub-path, each from its `moveto` to its `closepath`
 * @returns the `d` attribute of the `<path>`
 * @see REF-SVG2-PATHS
 */
export function formatSubPaths(subPaths: readonly string[]): string {
  return subPaths.join(SEPARATOR);
}

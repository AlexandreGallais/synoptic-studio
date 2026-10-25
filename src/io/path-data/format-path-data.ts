/** Separator between path-data commands and between sub-paths (SVG 2 §9.3.9, `wsp`). */
const SEPARATOR = " ";

/**
 * Path data written from its parts, commands or whole sub-paths, one white space between two: the
 * grammar separates them by `wsp` (§9.3.9), and "subsequent "moveto" commands … represent the
 * start of a new subpath" (§9.3.3), so closed sub-paths written one after the other draw as one
 * `<path>`. No part gives an empty `d`.
 *
 * @kind format
 * @param parts - the commands, or the sub-paths each from its `moveto` to its `closepath`
 * @returns the `d` attribute of the `<path>`
 */
export function formatPathData(parts: readonly string[]): string {
  return parts.join(SEPARATOR);
}

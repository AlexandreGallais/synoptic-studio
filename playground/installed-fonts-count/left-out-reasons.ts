import type { InstalledFaceLeftOut } from "./installed-face-left-out";

/**
 * The reasons an installed face or family is left out of the font list, in their one order
 * (`shapes.md`, Only usable fonts listed): a family takes the first one of its faces has,
 * and the reading line says them in it; each with how the line names it. A face's own reason
 * follows it too (`unlistedFaceReason`: not embeddable before no Latin letters).
 */
export const LEFT_OUT_REASONS: readonly (readonly [InstalledFaceLeftOut, string])[] = [
  ["not-embeddable", "not embeddable"],
  ["no-latin-letters", "no Latin letters"],
  ["unreadable", "unreadable"],
];

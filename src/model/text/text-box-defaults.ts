import type { Text } from "./text";

/**
 * The options of a text's box (F14) that a text gets until the designer changes them, so that a
 * text of F05 draws as before: it fits its content, its line height is automatic, its block is
 * not trimmed, it has no background, and the ellipsis is on (`DERIV-text-box`, step 6). The
 * others are read only in a box: no room until the box is sized, a word too long broken at a
 * character, no maximum number of lines, the block at the top of its box, the greedy wrap; the
 * background, when turned on, is yellow and opaque, as a highlighter (F14, provisional, to
 * confirm at VAL-015).
 */
export const TEXT_BOX_DEFAULTS: Pick<
  Text,
  | "backgroundColor"
  | "backgroundOpacity"
  | "boxHeight"
  | "boxWidth"
  | "hasBackground"
  | "hasEllipsis"
  | "isInBox"
  | "isTrimmed"
  | "lineHeight"
  | "longWord"
  | "maximumLines"
  | "verticalAlignment"
  | "wrapStyle"
> = {
  backgroundColor: "#ffff00",
  backgroundOpacity: 100,
  boxHeight: 0,
  boxWidth: 0,
  hasBackground: false,
  hasEllipsis: true,
  isInBox: false,
  isTrimmed: false,
  lineHeight: 0,
  longWord: "break",
  maximumLines: 0,
  verticalAlignment: "top",
  wrapStyle: "auto",
};

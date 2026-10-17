import type { FontMetrics } from "../../model";

/** Offset of `underlinePosition` in the `post` table. */
const UNDERLINE_POSITION_OFFSET = 8;

/** Offset of `underlineThickness` in the `post` table. */
const UNDERLINE_THICKNESS_OFFSET = 10;

/** Offset of `yStrikeoutSize` in the `OS/2` table. */
const STRIKEOUT_SIZE_OFFSET = 26;

/** Offset of `yStrikeoutPosition` in the `OS/2` table. */
const STRIKEOUT_POSITION_OFFSET = 28;

/** What a font says of its underline and strike-out strokes. */
export type TextDecorations = Pick<
  FontMetrics,
  "strikeoutPosition" | "strikeoutSize" | "underlinePosition" | "underlineThickness"
>;

/**
 * Reads where a font draws its underline, from `post`, and its strike-out stroke, from `OS/2`:
 * the top of each stroke from the baseline, and its thickness.
 *
 * @kind format
 * @param post - the font's `post` table, at least 12 bytes long
 * @param os2 - the font's `OS/2` table, at least 30 bytes long
 * @returns the underline's and the strike-out's position and thickness, font units
 * @see REF-OT-POST
 */
export function parseTextDecorations(
  post: Readonly<DataView>,
  os2: Readonly<DataView>,
): TextDecorations {
  return {
    strikeoutPosition: os2.getInt16(STRIKEOUT_POSITION_OFFSET),
    strikeoutSize: os2.getInt16(STRIKEOUT_SIZE_OFFSET),
    underlinePosition: post.getInt16(UNDERLINE_POSITION_OFFSET),
    underlineThickness: post.getInt16(UNDERLINE_THICKNESS_OFFSET),
  };
}

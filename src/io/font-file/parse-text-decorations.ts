import type { FontTables } from "./font-tables";
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
 * @param view - the font file
 * @param tables - where its tables sit
 * @returns the underline's and the strike-out's position and thickness, font units
 * @see REF-OT-POST
 */
export function parseTextDecorations(
  view: DataView,
  tables: Pick<FontTables, "os2" | "post">,
): TextDecorations {
  return {
    strikeoutPosition: view.getInt16(tables.os2.offset + STRIKEOUT_POSITION_OFFSET),
    strikeoutSize: view.getInt16(tables.os2.offset + STRIKEOUT_SIZE_OFFSET),
    underlinePosition: view.getInt16(tables.post.offset + UNDERLINE_POSITION_OFFSET),
    underlineThickness: view.getInt16(tables.post.offset + UNDERLINE_THICKNESS_OFFSET),
  };
}

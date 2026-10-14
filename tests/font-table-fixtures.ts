// The tables of a font file built byte by byte for the tests of `src/io/font-file/` (EN-031):
// `head`, `hhea`, `maxp`, `hmtx` and `cmap`, each field at its offset in the OpenType
// specification, and the test font of `[F05.AC1]`. Tests only: never part of the library.

import { TRUE_TYPE_VERSION, int16, tableOf, uint16, uint32 } from "./font-fixtures";
import { os2Table, postTable } from "./metrics-table-fixtures";

/** Version 0.5 of `maxp`, a font with CFF outlines or this test's. */
const MAXP_VERSION = 0x00_00_50_00;

/** Size of one `hmtx` record: advance width and left side bearing. */
const HMTX_RECORD_SIZE = 4;

/** `cmap` subtable format 4, segment mapping to delta values. */
const SEGMENT_MAPPING = 4;

/** `cmap` subtable format 12, segmented coverage. */
const SEGMENTED_COVERAGE = 12;

/**
 * A `head` table: 54 bytes, `unitsPerEm` at offset 18 (`REF-OT-HEAD`).
 *
 * @param unitsPerEm - from 16 to 16 384
 * @returns its bytes
 */
function headTable(unitsPerEm: number): number[] {
  return tableOf({
    fields: [
      { at: 0, bytes: uint32(TRUE_TYPE_VERSION) },
      { at: 18, bytes: uint16(unitsPerEm) },
    ],
    size: 54,
  });
}

/** The `hhea` fields written by `hheaTable`. */
export type HheaFields = {
  /** Typographic ascent, font units. */
  readonly ascender: number;
  /** Typographic descent, font units, negative below the baseline. */
  readonly descender: number;
  /** Typographic line gap, font units. */
  readonly lineGap: number;
  /** Number of records in `hmtx`. */
  readonly numberOfHMetrics: number;
};

/**
 * An `hhea` table: 36 bytes, ascender at 4, descender at 6, line gap at 8, `numberOfHMetrics` at 34
 * (`REF-OT-HHEA`).
 *
 * @param fields - the values to write
 * @returns its bytes
 */
export function hheaTable(fields: HheaFields): number[] {
  return tableOf({
    fields: [
      { at: 0, bytes: uint32(TRUE_TYPE_VERSION) },
      { at: 4, bytes: int16(fields.ascender) },
      { at: 6, bytes: int16(fields.descender) },
      { at: 8, bytes: int16(fields.lineGap) },
      { at: 34, bytes: uint16(fields.numberOfHMetrics) },
    ],
    size: 36,
  });
}

/**
 * A `maxp` table, version 0.5: its version, then the number of glyphs.
 *
 * @param glyphCount - number of glyphs of the font
 * @returns its bytes
 */
function maxpTable(glyphCount: number): number[] {
  return tableOf({
    fields: [
      { at: 0, bytes: uint32(MAXP_VERSION) },
      { at: 4, bytes: uint16(glyphCount) },
    ],
    size: 6,
  });
}

/**
 * An `hmtx` table: one record (advance, left side bearing 0) per advance, then a left side
 * bearing 0 for each remaining glyph (`REF-OT-HMTX`).
 *
 * @param advances - advance of each glyph with a record
 * @param remaining - number of glyphs past the records
 * @returns its bytes
 */
function hmtxTable(advances: readonly number[], remaining: number): number[] {
  return tableOf({
    fields: advances.map((advance, glyph) => ({
      at: HMTX_RECORD_SIZE * glyph,
      bytes: uint16(advance),
    })),
    size: HMTX_RECORD_SIZE * advances.length + 2 * remaining,
  });
}

/** One segment of a `cmap` format 4 subtable. */
export type CmapSegment = {
  /** First character code. */
  readonly start: number;
  /** Last character code. */
  readonly end: number;
  /** Added to the character code, or to the glyph read from the glyph array. */
  readonly delta: number;
  /** Glyphs of the segment's characters in the glyph array; empty when the delta alone maps them. */
  readonly glyphs: readonly number[];
};

/**
 * A `cmap` format 4 subtable (`REF-OT-CMAP`): a 14-byte header, then end codes, a pad, start
 * codes, deltas, range offsets and the glyph array; a range offset points from its own place to
 * the segment's first glyph in that array.
 *
 * @param segments - the segments, the last ending at 0xFFFF
 * @returns its bytes
 */
export function cmapFormat4(segments: readonly CmapSegment[]): number[] {
  const count = segments.length;
  const firstGlyphs = segments.map((unused, index) =>
    segments.slice(0, index).reduce((sum, segment) => sum + segment.glyphs.length, 0),
  );
  const rangeOffsets = segments.map((segment, index) =>
    segment.glyphs.length === 0 ? 0 : 2 * (count - index) + 2 * (firstGlyphs[index] ?? 0),
  );
  const body = [
    ...segments.flatMap((segment) => uint16(segment.end)),
    ...uint16(0),
    ...segments.flatMap((segment) => uint16(segment.start)),
    ...segments.flatMap((segment) => int16(segment.delta)),
    ...rangeOffsets.flatMap((offset) => uint16(offset)),
    ...segments.flatMap((segment) => segment.glyphs.flatMap((glyph) => uint16(glyph))),
  ];
  const header = { size: 14 };

  return tableOf({
    fields: [
      { at: 0, bytes: uint16(SEGMENT_MAPPING) },
      { at: 2, bytes: uint16(header.size + body.length) },
      { at: 6, bytes: uint16(2 * count) },
      { at: header.size, bytes: body },
    ],
    size: header.size + body.length,
  });
}

/** One group of a `cmap` format 12 subtable. */
export type CmapGroup = {
  /** First character code. */
  readonly start: number;
  /** Last character code. */
  readonly end: number;
  /** Glyph of the first character; the next characters take the next glyphs. */
  readonly glyph: number;
};

/**
 * A `cmap` format 12 subtable (`REF-OT-CMAP`): format, reserved, length, language, number of
 * groups, then the groups, 12 bytes each.
 *
 * @param groups - sequential map groups
 * @returns its bytes
 */
export function cmapFormat12(groups: readonly CmapGroup[]): number[] {
  const layout = { groupSize: 12, headerSize: 16 };
  const size = layout.headerSize + layout.groupSize * groups.length;

  return tableOf({
    fields: [
      { at: 0, bytes: uint16(SEGMENTED_COVERAGE) },
      { at: 4, bytes: uint32(size) },
      { at: 12, bytes: uint32(groups.length) },
      {
        at: layout.headerSize,
        bytes: groups.flatMap((group) => [
          ...uint32(group.start),
          ...uint32(group.end),
          ...uint32(group.glyph),
        ]),
      },
    ],
    size,
  });
}

/** One encoding record of a `cmap` table and its subtable. */
export type CmapEncoding = {
  /** Platform: 0 Unicode, 1 Macintosh, 3 Windows. */
  readonly platform: number;
  /** Encoding of the platform: for Windows, 0 Symbol, 1 BMP, 10 full repertoire. */
  readonly encoding: number;
  /** The subtable's bytes. */
  readonly subtable: readonly number[];
};

/**
 * A `cmap` table: version 0, its encoding records, 8 bytes each, then their subtables in the same
 * order (`REF-OT-CMAP`).
 *
 * @param encodings - the encoding records and their subtables
 * @returns its bytes
 */
export function cmapTable(encodings: readonly CmapEncoding[]): number[] {
  const layout = { headerSize: 4, recordSize: 8 };
  const offsets = encodings.map(
    (unused, index) =>
      layout.headerSize +
      layout.recordSize * encodings.length +
      encodings.slice(0, index).reduce((sum, encoding) => sum + encoding.subtable.length, 0),
  );

  return [
    ...uint16(0),
    ...uint16(encodings.length),
    ...encodings.flatMap((encoding, index) => [
      ...uint16(encoding.platform),
      ...uint16(encoding.encoding),
      ...uint32(offsets[index] ?? 0),
    ]),
    ...encodings.flatMap((encoding) => encoding.subtable),
  ];
}

/** The values of the test font of `[F05.AC1]`: glyph 0 is 500 units wide, glyph 1 600, glyph 2 700. */
const TEST_FONT = {
  ascender: 800,
  descender: -200,
  firstAdvance: 600,
  fsType: 4,
  glyphCount: 5,
  lastAdvance: 700,
  lineGap: 90,
  notdefAdvance: 500,
  strikeoutPosition: 300,
  strikeoutSize: 50,
  underlinePosition: -100,
  underlineThickness: 40,
  unitsPerEm: 1000,
};

/**
 * The tables of the test font of `[F05.AC1]`: 1000 units per em, ascender 800, descender −200,
 * line gap 90, three `hmtx` records (500, 600, 700) for five glyphs; A, B, C map to glyphs 1 to 3
 * by a delta of −64, `a` to glyph 4 through the glyph array; an underline at −100, 40 thick, a
 * strike-out at 300, 50 thick, `fsType` 4 (preview and print); no kerning.
 *
 * @returns the tables by tag
 */
export function testFontTables(): Record<string, readonly number[]> {
  const advances = [TEST_FONT.notdefAdvance, TEST_FONT.firstAdvance, TEST_FONT.lastAdvance];

  return {
    cmap: cmapTable([
      {
        encoding: 1,
        platform: 3,
        subtable: cmapFormat4([
          { delta: -64, end: 67, glyphs: [], start: 65 },
          { delta: 0, end: 97, glyphs: [TEST_FONT.glyphCount - 1], start: 97 },
          { delta: 1, end: 0xff_ff, glyphs: [], start: 0xff_ff },
        ]),
      },
    ]),
    head: headTable(TEST_FONT.unitsPerEm),
    hhea: hheaTable({ ...TEST_FONT, numberOfHMetrics: advances.length }),
    hmtx: hmtxTable(advances, TEST_FONT.glyphCount - advances.length),
    maxp: maxpTable(TEST_FONT.glyphCount),
    "OS/2": os2Table(TEST_FONT),
    post: postTable(TEST_FONT.underlinePosition, TEST_FONT.underlineThickness),
  };
}

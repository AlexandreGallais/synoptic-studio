import { elementAddress, isSpanWithin, rowMajorOffset } from "../../math";

import { parseClassDefinition } from "./parse-class-definition";
import { parseCoverage } from "./parse-coverage";
import { parseXAdvance } from "./parse-x-advance";
import { valueFormatToSize } from "./value-format-to-size";

import type { ClassKerningRule, KerningRow } from "../../model";

/** Offsets of the fields of a PairPos format 2 subtable. */
const FIELDS = {
  class1Count: 12,
  class1Records: 16,
  class2Count: 14,
  classDef1: 8,
  classDef2: 10,
  coverage: 2,
  valueFormat1: 4,
  valueFormat2: 6,
};

/**
 * Reads the kerning of each pair of classes of a PairPos format 2 subtable: Class1 records by
 * first class, each holding a Class2 record per second class, class 0 included; the first glyph's
 * `xAdvance` is the kerning. Records of no bytes, both value formats 0, hold no kerning: each first
 * class gets an empty row, read as 0, never a value per pair of classes, whatever the counts.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param subtable - where the subtable starts in it, its header within the table
 * @returns the kerning by first class, then second class; undefined when the records pass the
 * table's end
 * @see REF-OT-GPOS
 */
function parseClassValues(gpos: DataView, subtable: number): readonly KerningRow[] | undefined {
  const firstFormat = gpos.getUint16(subtable + FIELDS.valueFormat1);
  const recordSize =
    valueFormatToSize(firstFormat) +
    valueFormatToSize(gpos.getUint16(subtable + FIELDS.valueFormat2));
  const firstCount = gpos.getUint16(subtable + FIELDS.class1Count);
  const secondCount = recordSize === 0 ? 0 : gpos.getUint16(subtable + FIELDS.class2Count);
  const records = subtable + FIELDS.class1Records;

  return isSpanWithin(records, recordSize * firstCount * secondCount, gpos.byteLength)
    ? Array.from({ length: firstCount }, (unused, first) =>
        Array.from({ length: secondCount }, (unusedRecord, second) =>
          parseXAdvance(
            gpos,
            elementAddress(records, recordSize, rowMajorOffset(first, second, secondCount)),
            firstFormat,
          ),
        ),
      )
    : undefined;
}

/**
 * Reads the class of each first glyph asked that the subtable covers, class 0 when the first
 * class definition leaves it out.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param subtable - where the subtable starts in it, its header within the table
 * @param glyphs - the glyphs asked
 * @returns the class of each covered first glyph, by glyph; undefined when the coverage or the
 * class definition passes the table's end
 * @see REF-OT-GPOS
 */
function parseFirstClasses(
  gpos: DataView,
  subtable: number,
  glyphs: ReadonlySet<number>,
): Readonly<Record<number, number>> | undefined {
  const covered = parseCoverage(
    gpos,
    subtable + gpos.getUint16(subtable + FIELDS.coverage),
    glyphs,
  );
  const classes = parseClassDefinition(
    gpos,
    subtable + gpos.getUint16(subtable + FIELDS.classDef1),
    glyphs,
  );

  return covered === undefined || classes === undefined
    ? undefined
    : Object.fromEntries(Object.keys(covered).map((glyph) => [glyph, classes[Number(glyph)] ?? 0]));
}

/**
 * Reads the parts of a PairPos format 2 subtable for some glyphs: the classes of the first glyphs
 * it covers, those of the second glyphs, and the values.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param subtable - where the subtable starts in it, its header within the table
 * @param glyphs - the glyphs asked, first and second
 * @returns the rule by classes; undefined when a part passes the table's end
 * @see REF-OT-GPOS
 */
function parseClassParts(
  gpos: DataView,
  subtable: number,
  glyphs: ReadonlySet<number>,
): ClassKerningRule | undefined {
  const firstClasses = parseFirstClasses(gpos, subtable, glyphs);
  const secondClasses = parseClassDefinition(
    gpos,
    subtable + gpos.getUint16(subtable + FIELDS.classDef2),
    glyphs,
  );
  const values = parseClassValues(gpos, subtable);

  return firstClasses === undefined || secondClasses === undefined || values === undefined
    ? undefined
    : { firstClasses, kind: "classes", secondClasses, values };
}

/**
 * Reads a pair adjustment subtable of format 2 for some glyphs: the classes of the first glyphs it
 * covers, those of the second glyphs, and the kerning of each pair of classes.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param subtable - where the subtable starts in it
 * @param glyphs - the glyphs asked, first and second
 * @returns the rule by classes; undefined when a part of it passes the table's end
 * @see REF-OT-GPOS
 */
export function parsePairClassesRule(
  gpos: DataView,
  subtable: number,
  glyphs: ReadonlySet<number>,
): ClassKerningRule | undefined {
  return isSpanWithin(subtable, FIELDS.class1Records, gpos.byteLength)
    ? parseClassParts(gpos, subtable, glyphs)
    : undefined;
}

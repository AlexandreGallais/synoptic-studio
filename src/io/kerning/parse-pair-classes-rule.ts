import { parseClassDefinition } from "./parse-class-definition";
import { parseCoverage } from "./parse-coverage";
import { parseXAdvance } from "./parse-x-advance";
import { valueFormatToSize } from "./value-format-to-size";

import type { ClassKerningRule } from "../../model";

/** The kerning of one class with every class, font units. */
type Row = readonly number[];

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
 * `xAdvance` is the kerning.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @returns the kerning by first class, then second class
 * @see REF-OT-GPOS
 */
function parseClassValues(view: DataView, subtable: number): readonly Row[] {
  const firstFormat = view.getUint16(subtable + FIELDS.valueFormat1);
  const recordSize =
    valueFormatToSize(firstFormat) +
    valueFormatToSize(view.getUint16(subtable + FIELDS.valueFormat2));
  const secondCount = view.getUint16(subtable + FIELDS.class2Count);
  const records = subtable + FIELDS.class1Records;

  return Array.from({ length: view.getUint16(subtable + FIELDS.class1Count) }, (unused, first) =>
    Array.from({ length: secondCount }, (unusedRecord, second) =>
      parseXAdvance(view, records + recordSize * (secondCount * first + second), firstFormat),
    ),
  );
}

/**
 * Reads a pair adjustment subtable of format 2: the classes of the first glyphs it covers, those
 * of the second glyphs, and the kerning of each pair of classes.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @returns the rule by classes
 * @see REF-OT-GPOS
 */
export function parsePairClassesRule(view: DataView, subtable: number): ClassKerningRule {
  const firstClasses = parseClassDefinition(
    view,
    subtable + view.getUint16(subtable + FIELDS.classDef1),
  );
  const covered = parseCoverage(view, subtable + view.getUint16(subtable + FIELDS.coverage));

  return {
    firstClasses: Object.fromEntries(covered.map((glyph) => [glyph, firstClasses[glyph] ?? 0])),
    kind: "classes",
    secondClasses: parseClassDefinition(
      view,
      subtable + view.getUint16(subtable + FIELDS.classDef2),
    ),
    values: parseClassValues(view, subtable),
  };
}

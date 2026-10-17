import { elementAddress, isSpanWithin } from "../../math";

import { parsePairClassesRule } from "./parse-pair-classes-rule";
import { parsePairGlyphsRule } from "./parse-pair-glyphs-rule";

import type { KerningLookup, KerningRule } from "../../model";

/** The `GPOS` table's bytes, only read. */
type GposTable = Readonly<DataView>;

/** GPOS lookup type 2, pair adjustment. */
const PAIR_ADJUSTMENT = 2;

/** GPOS lookup type 9, an extension pointing to a subtable of another type. */
const EXTENSION = 9;

/** PairPos format 1, adjustments for glyph pairs. */
const GLYPH_PAIRS_FORMAT = 1;

/** PairPos format 2, class pair adjustment. */
const CLASS_PAIRS_FORMAT = 2;

/** Size of a Lookup table's header: type, flag and subtable count. */
const LOOKUP_HEADER_SIZE = 6;

/** Offset of `subTableCount` in a Lookup table. */
const SUBTABLE_COUNT_OFFSET = 4;

/** Size of an extension subtable: format, extension lookup type, then a 32-bit offset. */
const EXTENSION_SIZE = 8;

/** Offset of `extensionOffset` in an extension subtable. */
const EXTENSION_OFFSET_FIELD = 4;

/** Size of a subtable's offset in the Lookup table, and of a subtable's format field. */
const FIELD_SIZE = 2;

/** A lookup subtable with its effective type. */
type Subtable = {
  /** The lookup type of the subtable's data. */
  readonly type: number;
  /** Where its data starts in the table. */
  readonly offset: number;
};

/**
 * Where a lookup subtable's data sits, an extension replaced by the subtable it points to, of its
 * `extensionLookupType`.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param type - the lookup's type
 * @param subtable - where the subtable starts in the table
 * @returns its effective type and offset; undefined for an extension passing the table's end
 * @see REF-OT-GPOS
 */
function parseEffectiveSubtable(
  gpos: GposTable,
  type: number,
  subtable: number,
): Subtable | undefined {
  if (type !== EXTENSION) {
    return { offset: subtable, type };
  }

  return isSpanWithin(subtable, EXTENSION_SIZE, gpos.byteLength)
    ? {
        offset: subtable + gpos.getUint32(subtable + EXTENSION_OFFSET_FIELD),
        type: gpos.getUint16(subtable + 2),
      }
    : undefined;
}

/**
 * Reads where a lookup's subtables sit, with their effective types.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param lookup - where the Lookup table starts in the table
 * @returns its subtables, in order; undefined when the lookup or an extension passes the
 * table's end
 * @see REF-OT-LAYOUT-COMMON
 */
function parseLookupSubtables(gpos: GposTable, lookup: number): readonly Subtable[] | undefined {
  const count = isSpanWithin(lookup, LOOKUP_HEADER_SIZE, gpos.byteLength)
    ? gpos.getUint16(lookup + SUBTABLE_COUNT_OFFSET)
    : Infinity;
  const subtables = isSpanWithin(
    lookup,
    elementAddress(LOOKUP_HEADER_SIZE, FIELD_SIZE, count),
    gpos.byteLength,
  )
    ? Array.from({ length: count }, (unused, index) =>
        parseEffectiveSubtable(
          gpos,
          gpos.getUint16(lookup),
          lookup + gpos.getUint16(elementAddress(lookup + LOOKUP_HEADER_SIZE, FIELD_SIZE, index)),
        ),
      )
    : [undefined];
  const found = subtables.filter((subtable): subtable is Subtable => subtable !== undefined);

  return found.length < subtables.length ? undefined : found;
}

/**
 * A rule as a list: one rule, or undefined when it could not be read.
 *
 * @kind format
 * @param rule - the rule read, if any
 * @returns the rule alone in a list; undefined when it could not be read
 * @see REF-OT-GPOS
 */
function ruleToList(rule: KerningRule | undefined): readonly KerningRule[] | undefined {
  return rule === undefined ? undefined : [rule];
}

/**
 * Reads the pair adjustment rule of a subtable for some glyphs: format 1 by glyph pairs, format 2
 * by classes; another format gives no rule.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param subtable - where the subtable starts in the table
 * @param glyphs - the glyphs asked, first and second
 * @returns its rule, or none; undefined when it passes the table's end
 * @see REF-OT-GPOS
 */
function subtableToRules(
  gpos: GposTable,
  subtable: number,
  glyphs: ReadonlySet<number>,
): readonly KerningRule[] | undefined {
  if (!isSpanWithin(subtable, FIELD_SIZE, gpos.byteLength)) {
    return undefined;
  }

  switch (gpos.getUint16(subtable)) {
    case GLYPH_PAIRS_FORMAT: {
      return ruleToList(parsePairGlyphsRule(gpos, subtable, glyphs));
    }

    case CLASS_PAIRS_FORMAT: {
      return ruleToList(parsePairClassesRule(gpos, subtable, glyphs));
    }

    default: {
      return [];
    }
  }
}

/**
 * Reads the rules of a lookup's pair adjustment subtables; a subtable of another type gives no
 * rule.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param subtables - the lookup's subtables, with their effective types
 * @param glyphs - the glyphs asked, first and second
 * @returns the rules, in subtable order; undefined when one passes the table's end
 * @see REF-OT-GPOS
 */
function subtablesToLookup(
  gpos: GposTable,
  subtables: readonly Subtable[],
  glyphs: ReadonlySet<number>,
): KerningLookup | undefined {
  const rules = subtables
    .filter((subtable) => subtable.type === PAIR_ADJUSTMENT)
    .map((subtable) => subtableToRules(gpos, subtable.offset, glyphs));

  const found = rules.filter((rule): rule is readonly KerningRule[] => rule !== undefined);

  return found.length < rules.length ? undefined : found.flat();
}

/**
 * Reads a GPOS lookup as kerning for some glyphs: its pair adjustment subtables in order,
 * extensions followed.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param lookup - where the Lookup table starts in the table
 * @param glyphs - the glyphs asked, first and second
 * @returns its rules, in subtable order; undefined when a part passes the table's end
 * @see REF-OT-LAYOUT-COMMON
 */
export function parsePairLookup(
  gpos: GposTable,
  lookup: number,
  glyphs: ReadonlySet<number>,
): KerningLookup | undefined {
  const subtables = parseLookupSubtables(gpos, lookup);

  return subtables === undefined ? undefined : subtablesToLookup(gpos, subtables, glyphs);
}

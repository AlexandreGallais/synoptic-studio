import { parsePairClassesRule } from "./parse-pair-classes-rule";
import { parsePairGlyphsRule } from "./parse-pair-glyphs-rule";

import type { KerningLookup, KerningRule } from "../../model";

/** GPOS lookup type 2, pair adjustment. */
const PAIR_ADJUSTMENT = 2;

/** GPOS lookup type 9, an extension pointing to a subtable of another type. */
const EXTENSION = 9;

/** Offset of `subtableOffsets` in a Lookup table. */
const SUBTABLE_OFFSETS = 6;

/** Offset of `subTableCount` in a Lookup table. */
const SUBTABLE_COUNT_OFFSET = 4;

/** Offset of `extensionOffset` in an extension subtable. */
const EXTENSION_OFFSET_FIELD = 4;

/** A lookup subtable with its effective type. */
type Subtable = {
  /** The lookup type of the subtable's data. */
  readonly type: number;
  /** Where its data starts in the file. */
  readonly offset: number;
};

/**
 * Where a lookup subtable's data sits, an extension replaced by the subtable it points to, of its
 * `extensionLookupType`.
 *
 * @kind format
 * @param view - the font file
 * @param type - the lookup's type
 * @param subtable - where the subtable starts in the file
 * @returns its effective type and offset
 * @see REF-OT-GPOS
 */
function parseEffectiveSubtable(view: DataView, type: number, subtable: number): Subtable {
  return type === EXTENSION
    ? {
        offset: subtable + view.getUint32(subtable + EXTENSION_OFFSET_FIELD),
        type: view.getUint16(subtable + 2),
      }
    : { offset: subtable, type };
}

/**
 * Reads the pair adjustment rule of a subtable: format 1 by glyph pairs, format 2 by classes.
 *
 * @kind format
 * @param view - the font file
 * @param subtable - where the subtable starts in the file
 * @returns its rule; none for another format
 * @see REF-OT-GPOS
 */
function parseRules(view: DataView, subtable: number): readonly KerningRule[] {
  switch (view.getUint16(subtable)) {
    case 1: {
      return [parsePairGlyphsRule(view, subtable)];
    }

    case PAIR_ADJUSTMENT: {
      return [parsePairClassesRule(view, subtable)];
    }

    default: {
      return [];
    }
  }
}

/**
 * Reads a GPOS lookup as kerning: its pair adjustment subtables in order, extensions followed;
 * a subtable of another type gives no rule.
 *
 * @kind format
 * @param view - the font file
 * @param lookup - where the Lookup table starts in the file
 * @returns its rules, in subtable order
 * @see REF-OT-LAYOUT-COMMON
 */
export function parsePairLookup(view: DataView, lookup: number): KerningLookup {
  const type = view.getUint16(lookup);

  return Array.from({ length: view.getUint16(lookup + SUBTABLE_COUNT_OFFSET) }, (unused, index) =>
    parseEffectiveSubtable(
      view,
      type,
      lookup + view.getUint16(lookup + SUBTABLE_OFFSETS + 2 * index),
    ),
  )
    .filter((subtable) => subtable.type === PAIR_ADJUSTMENT)
    .flatMap((subtable) => parseRules(view, subtable.offset));
}

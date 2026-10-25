import { parseNameRecords } from "./parse-name-records";
import { parseNameString } from "./parse-name-string";

import type { NameRecord } from "./name-record";

/** A name read, and its rank in the choice: the lowest is chosen. */
type NameCandidate = {
  /** Its rank: 0 Windows English (United States), 1 Macintosh English, 2 Unicode, 3 another. */
  readonly rank: number;
  /** Its string. */
  readonly text: string;
};

/**
 * The ranks of the English names, by platform and language: Windows language 0x0409, English
 * (United States) (`REF-MS-LCID`), then Macintosh language 0, English (`REF-APPLE-NAME`).
 */
const ENGLISH_RANKS: Readonly<Record<string, number>> = { "1/0": 1, "3/1033": 0 };

/** The rank of a Unicode platform name, which has no language (`REF-OT-NAME`). */
const UNICODE_RANK = 2;

/** The rank of any other name: another language of Windows or Macintosh. */
const OTHER_RANK = 3;

/** The platform whose names carry no language: 0, Unicode. */
const UNICODE_PLATFORM = 0;

/**
 * Where a name comes in the choice: English first — Windows English (United States), then
 * Macintosh English —, then a Unicode platform name, then any other language.
 *
 * @kind format
 * @param record - a name record of the ID asked
 * @returns its rank, the lowest chosen first
 */
function nameRecordToRank(record: NameRecord): number {
  return (
    ENGLISH_RANKS[`${String(record.platform)}/${String(record.language)}`] ??
    (record.platform === UNICODE_PLATFORM ? UNICODE_RANK : OTHER_RANK)
  );
}

/**
 * Reads the names of one name ID that the library can decode, each with its rank.
 *
 * @kind format
 * @param name - the font's `name` table
 * @param nameId - the name ID asked
 * @returns the names of that ID, not empty, in the table's order
 */
function parseNameCandidates(name: Readonly<DataView>, nameId: number): readonly NameCandidate[] {
  return parseNameRecords(name)
    .filter((record) => record.nameId === nameId)
    .map((record) => ({ rank: nameRecordToRank(record), text: parseNameString(name, record) }))
    .filter((candidate): candidate is NameCandidate => (candidate.text ?? "") !== "");
}

/**
 * Reads a name of a font from its `name` table: of the strings of that name ID the library can
 * decode, the English one first — Windows English (United States), then Macintosh English —,
 * then a Unicode platform string, then any other, the first in the table's order among equals.
 * Every read is checked against the table: a damaged table gives no name, never an error.
 *
 * @kind format
 * @param name - the font's `name` table
 * @param nameId - the name ID: 1 family, 5 version, 16 typographic family
 * @returns the name; undefined when the table holds no readable string of that ID
 */
export function parseFontName(name: Readonly<DataView>, nameId: number): string | undefined {
  return parseNameCandidates(name, nameId).reduce<NameCandidate | undefined>(
    (best, candidate) => (best === undefined || candidate.rank < best.rank ? candidate : best),
    undefined,
  )?.text;
}

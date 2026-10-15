/** Size of the `name` table's header: `version`, `count` and `storageOffset`. */
const NAME_HEADER_SIZE = 6;

/** Offset of `count` in the `name` table. */
const COUNT_OFFSET = 2;

/** Offset of `storageOffset` in the `name` table. */
const STORAGE_OFFSET = 4;

/** Size of one name record: six fields of two bytes. */
const NAME_RECORD_SIZE = 12;

/** Offsets of a name record's fields, from the record's start. */
const RECORD_FIELDS = { language: 4, length: 8, nameId: 6, offset: 10, platform: 0 };

/** Platform 3, Windows, whose strings "must be encoded in UTF-16BE". */
const WINDOWS_PLATFORM = 3;

/** Language 0x0409 of platform 3: English (United States). */
const ENGLISH_US = 0x04_09;

/** Bytes of one UTF-16 code unit. */
const CODE_UNIT_SIZE = Uint16Array.BYTES_PER_ELEMENT;

/** One name record of the `name` table, its string placed from the table's start. */
type NameRecord = {
  /** Platform: 1 Macintosh, 3 Windows. */
  readonly platform: number;
  /** Language of the platform. */
  readonly language: number;
  /** Name ID: 1 family, 16 typographic family. */
  readonly nameId: number;
  /** Where the string starts, from the table's start, in bytes. */
  readonly offset: number;
  /** Length of the string, in bytes. */
  readonly length: number;
};

/**
 * How many name records the `name` table holds: none when its header, or its records, do not fit
 * in it.
 *
 * @kind format
 * @param name - the font's `name` table
 * @returns its number of records
 * @see REF-OT-NAME
 */
function parseRecordCount(name: DataView): number {
  const count = name.byteLength >= NAME_HEADER_SIZE ? name.getUint16(COUNT_OFFSET) : 0;

  return NAME_HEADER_SIZE + NAME_RECORD_SIZE * count > name.byteLength ? 0 : count;
}

/**
 * Reads the name records of the `name` table, each string's offset counted from the storage
 * area's start (`storageOffset`); a record whose string passes the table's end is left out.
 *
 * @kind format
 * @param name - the font's `name` table
 * @returns its records whose string lies in the table
 * @see REF-OT-NAME
 */
function parseNameRecords(name: DataView): readonly NameRecord[] {
  return Array.from({ length: parseRecordCount(name) }, (unused, index): NameRecord => {
    const at = NAME_HEADER_SIZE + NAME_RECORD_SIZE * index;

    return {
      language: name.getUint16(at + RECORD_FIELDS.language),
      length: name.getUint16(at + RECORD_FIELDS.length),
      nameId: name.getUint16(at + RECORD_FIELDS.nameId),
      offset: name.getUint16(STORAGE_OFFSET) + name.getUint16(at + RECORD_FIELDS.offset),
      platform: name.getUint16(at + RECORD_FIELDS.platform),
    };
  }).filter((record) => record.offset + record.length <= name.byteLength);
}

/**
 * Reads the name records of one name ID on the Windows platform, each of one byte at least.
 *
 * @kind format
 * @param name - the font's `name` table
 * @param nameId - the name ID asked
 * @returns its Windows records whose string lies in the table, not empty, in the table's order
 * @see REF-OT-NAME
 */
function parseWindowsNames(name: DataView, nameId: number): readonly NameRecord[] {
  return parseNameRecords(name).filter(
    (record) =>
      record.platform === WINDOWS_PLATFORM && record.nameId === nameId && record.length > 0,
  );
}

/**
 * Reads a name's string, in UTF-16BE: one code unit per two bytes, big-endian, a character past
 * the Basic Multilingual Plane as its surrogate pair.
 *
 * @kind format
 * @param name - the font's `name` table
 * @param record - the name's record
 * @returns its string
 * @see REF-OT-NAME
 */
function parseUtf16Name(name: DataView, record: NameRecord): string {
  return String.fromCodePoint(
    ...Array.from({ length: Math.floor(record.length / CODE_UNIT_SIZE) }, (unused, index) =>
      name.getUint16(record.offset + CODE_UNIT_SIZE * index),
    ),
  );
}

/**
 * Reads a name of a font from its `name` table: the Windows string of that name ID, in
 * English (United States) when the table has one, else the first Windows string of that ID.
 * Every read is checked against the table: a damaged table gives no name, never an error.
 *
 * @kind format
 * @param name - the font's `name` table
 * @param nameId - the name ID: 1 family, 16 typographic family
 * @returns the name; undefined when the table holds no Windows string of that ID
 * @see REF-OT-NAME
 */
export function parseFontName(name: DataView, nameId: number): string | undefined {
  const records = parseWindowsNames(name, nameId);
  const chosen = records.find((record) => record.language === ENGLISH_US) ?? records[0];

  return chosen === undefined ? undefined : parseUtf16Name(name, chosen);
}

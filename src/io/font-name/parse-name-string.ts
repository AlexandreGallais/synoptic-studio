import { elementAddress, integerQuotient } from "../../math";

import type { NameRecord } from "./name-record";

/** Bytes of one UTF-16 code unit. */
const CODE_UNIT_SIZE = Uint16Array.BYTES_PER_ELEMENT;

/** The bytes below it are ASCII, which the first 128 characters of the Macintosh Roman encoding are. */
const ASCII_END = 0x80;

/** A reader of a name's string, in its platform's encoding. */
type NameDecoder = (name: DataView, record: NameRecord) => string | undefined;

/**
 * Reads a name's string in UTF-16BE: one code unit per two bytes, big-endian, a character past
 * the Basic Multilingual Plane as its surrogate pair; an odd last byte is not a code unit and is
 * left out.
 *
 * @kind format
 * @param name - the font's `name` table
 * @param record - the name's record, its string within the table
 * @returns its string
 * @see REF-OT-NAME
 */
function parseUtf16Name(name: DataView, record: NameRecord): string {
  return String.fromCodePoint(
    ...Array.from({ length: integerQuotient(record.length, CODE_UNIT_SIZE) }, (unused, index) =>
      name.getUint16(elementAddress(record.offset, CODE_UNIT_SIZE, index)),
    ),
  );
}

/**
 * Reads a Macintosh name in the Roman script when all its bytes are ASCII: the first 128
 * characters of the Macintosh Roman encoding are ASCII's (`REF-WIKI-MAC-ROMAN`), the others are not read.
 *
 * @kind format
 * @param name - the font's `name` table
 * @param record - the name's record, its string within the table
 * @returns its string; undefined when a byte is 128 or more
 * @see REF-APPLE-NAME
 */
function parseAsciiName(name: DataView, record: NameRecord): string | undefined {
  const bytes = Array.from({ length: record.length }, (unused, index) =>
    name.getUint8(record.offset + index),
  );

  return bytes.every((byte) => byte < ASCII_END) ? String.fromCodePoint(...bytes) : undefined;
}

/**
 * The readers of the names the library reads, by platform and encoding: Unicode (platform 0,
 * every encoding) and Windows Unicode (platform 3, encodings 1 and 10) in UTF-16BE; Macintosh
 * Roman (platform 1, encoding 0) when ASCII. Windows encodings 3, 4 and 5 use code pages, and
 * the other Macintosh scripts their own encodings: not read.
 */
const NAME_DECODERS: Readonly<Record<string, NameDecoder>> = {
  "0/0": parseUtf16Name,
  "0/1": parseUtf16Name,
  "0/2": parseUtf16Name,
  "0/3": parseUtf16Name,
  "0/4": parseUtf16Name,
  "1/0": parseAsciiName,
  "3/1": parseUtf16Name,
  "3/10": parseUtf16Name,
};

/**
 * Reads a name's string in the encoding of its platform: UTF-16BE for the Unicode platform and
 * Windows' Unicode encodings 1 and 10 (`REF-OT-NAME`, `REF-APPLE-NAME`), ASCII for a Macintosh
 * Roman name.
 *
 * @kind format
 * @param name - the font's `name` table
 * @param record - the name's record, its string within the table
 * @returns its string; undefined for an encoding the library does not read
 * @see REF-OT-NAME
 */
export function parseNameString(name: DataView, record: NameRecord): string | undefined {
  return NAME_DECODERS[`${String(record.platform)}/${String(record.encoding)}`]?.(name, record);
}

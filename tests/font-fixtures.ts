// A font file built byte by byte for the tests of `src/io/font-file/` and `src/io/kerning/`
// (EN-031): the table directory and the tables of the OpenType specification, each field written
// at its offset so that every value read back is known by hand. Tests only: never part of the
// library.

import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

/**
 * Inter Regular 4.1, the release's `extras/ttf/Inter-Regular.ttf` unchanged (`REF-INTER-RELEASE`),
 * with its licence beside it in `inter-license.txt` (`REF-INTER-LICENSE`): a real font file that
 * every machine reads (EN-062).
 */
export const INTER_REGULAR_FILE: Uint8Array = readFileSync(
  join(import.meta.dirname, "inter-regular.ttf"),
);

/** Arial 5.01 of macOS, the font of the cross-check of `DERIV-text-width`. */
export const ARIAL_PATH = "/System/Library/Fonts/Supplemental/Arial.ttf";

/**
 * A font file of the machine, if it has it: tests that need one skip without it.
 *
 * @param path - where the file sits
 * @returns its bytes; undefined when the machine has no such file
 */
export function systemFont(path: string): Uint8Array | undefined {
  return existsSync(path) ? readFileSync(path) : undefined;
}

/** `sfntVersion` of a font with TrueType outlines (`REF-OT-OTFF`). */
export const TRUE_TYPE_VERSION = 0x00_01_00_00;

/** `sfntVersion` of a font with CFF outlines, the tag `OTTO` (`REF-OT-OTFF`). */
export const OPEN_TYPE_VERSION = 0x4f_54_54_4f;

/** Values of one byte. */
const BYTE = 0x1_00;

/** Values of two bytes. */
const WORD = 0x1_00_00;

/** Tables start at a multiple of four bytes (`REF-OT-OTFF`). */
const TABLE_ALIGNMENT = 4;

/** Sizes and offsets of the table directory (`REF-OT-OTFF`). */
const DIRECTORY = { headerSize: 12, recordSize: 16 };

/** One field of a table: its bytes, at an offset. */
type TableField = {
  /** Offset of the field from the start of the table. */
  readonly at: number;
  /** The field's bytes. */
  readonly bytes: readonly number[];
};

/** A table as its size and its fields; the bytes no field writes are zeros. */
export type TableLayout = {
  /** Size of the table, in bytes. */
  readonly size: number;
  /** Its fields. */
  readonly fields: readonly TableField[];
};

/**
 * An unsigned 16-bit integer, big-endian.
 *
 * @param value - from 0 to 65 535
 * @returns its two bytes
 */
export function uint16(value: number): number[] {
  return [Math.floor(value / BYTE) % BYTE, value % BYTE];
}

/**
 * A signed 16-bit integer, big-endian, in two's complement.
 *
 * @param value - from −32 768 to 32 767
 * @returns its two bytes
 */
export function int16(value: number): number[] {
  return uint16(((value % WORD) + WORD) % WORD);
}

/**
 * An unsigned 32-bit integer, big-endian.
 *
 * @param value - from 0 to 2³² − 1
 * @returns its four bytes
 */
export function uint32(value: number): number[] {
  return [...uint16(Math.floor(value / WORD)), ...uint16(value % WORD)];
}

/**
 * The bytes of a tag.
 *
 * @param tag - four ASCII characters
 * @returns their codes
 */
export function tagBytes(tag: string): number[] {
  return Array.from({ length: tag.length }, (unused, index) => tag.codePointAt(index) ?? 0);
}

/**
 * The bytes of a table from its layout.
 *
 * @param layout - its size and fields
 * @returns its bytes
 */
export function tableOf(layout: TableLayout): number[] {
  const bytes = Array.from({ length: layout.size }, () => 0);

  for (const field of layout.fields) {
    bytes.splice(field.at, field.bytes.length, ...field.bytes);
  }

  return bytes;
}

/**
 * A view of bytes, as the readers of `src/io/` take a font file.
 *
 * @param bytes - the file's bytes, each from 0 to 255
 * @returns a view of a copy of them
 */
export function viewOf(bytes: readonly number[]): DataView {
  return new DataView(Uint8Array.from(bytes).buffer);
}

/**
 * A view of the first bytes of a table, as a table cut short.
 *
 * @param bytes - the table's bytes
 * @param size - how many bytes are kept; a negative size cuts that many from the end
 * @returns a view of the bytes kept
 */
export function viewOfFirst(bytes: readonly number[], size: number): DataView {
  return viewOf(bytes.slice(0, size));
}

/**
 * A font file: the table directory, then each table at a 4-byte boundary, in the order given.
 *
 * @param sfntVersion - `TRUE_TYPE_VERSION` or `OPEN_TYPE_VERSION`
 * @param tables - the bytes of each table, by tag
 * @returns the file's bytes
 */
export function fontFile(
  sfntVersion: number,
  tables: Readonly<Record<string, readonly number[]>>,
): Uint8Array {
  const tags = Object.keys(tables);
  const bodies = tags.map((tag) => tables[tag] ?? []);
  const padded = bodies.map((body) =>
    tableOf({
      fields: [{ at: 0, bytes: body }],
      size: Math.ceil(body.length / TABLE_ALIGNMENT) * TABLE_ALIGNMENT,
    }),
  );
  const starts = padded.map(
    (unused, index) =>
      DIRECTORY.headerSize +
      DIRECTORY.recordSize * tags.length +
      padded.slice(0, index).reduce((sum, body) => sum + body.length, 0),
  );
  const records = tags.flatMap((tag, index) => [
    ...tagBytes(tag),
    ...uint32(0),
    ...uint32(starts[index] ?? 0),
    ...uint32(bodies[index]?.length ?? 0),
  ]);

  return Uint8Array.from([
    ...tableOf({
      fields: [
        { at: 0, bytes: uint32(sfntVersion) },
        { at: 4, bytes: uint16(tags.length) },
      ],
      size: DIRECTORY.headerSize,
    }),
    ...records,
    ...padded.flat(),
  ]);
}

/** Helvetica of macOS, a collection (`.ttc`) of 6 faces, the font of US-046's examples. */
export const HELVETICA_COLLECTION_PATH = "/System/Library/Fonts/Helvetica.ttc";

/** `sfntVersion` of a TrueType font as Apple's systems tag it, `true` (`REF-APPLE-TT-SCALER`). */
export const APPLE_TRUE_TYPE_VERSION = 0x74_72_75_65;

/**
 * A font collection (`.ttc`, version 1.0): its header — the tag `ttcf`, the versions, `numFonts`
 * and one offset per face — then each face's file whole, its table offsets moved to count from
 * the collection's start, as `REF-OT-OTFF` asks. Each face starts on a four-byte boundary, the
 * faces of `fontFile` being padded.
 *
 * @param faces - the file of each face, as `fontFile` writes it
 * @returns the collection's bytes
 */
export function fontCollection(faces: readonly Uint8Array[]): Uint8Array {
  const layout = { numTables: 4, offsetField: 8, offsetSize: 4 };
  const headerSize = DIRECTORY.headerSize + layout.offsetSize * faces.length;
  const bases = faces.map(
    (unused, index) =>
      headerSize + faces.slice(0, index).reduce((sum, face) => sum + face.length, 0),
  );
  const moved = faces.map((face, index) => {
    const view = new DataView(face.buffer, face.byteOffset, face.byteLength);
    const fields = Array.from(
      { length: view.getUint16(layout.numTables) },
      (unused, record) => DIRECTORY.headerSize + DIRECTORY.recordSize * record + layout.offsetField,
    );

    return fields.reduce<number[]>(
      (bytes, field) => [
        ...bytes.slice(0, field),
        ...uint32(view.getUint32(field) + (bases[index] ?? 0)),
        ...bytes.slice(field + layout.offsetSize),
      ],
      [...face],
    );
  });

  return Uint8Array.from([
    ...tagBytes("ttcf"),
    ...uint16(1),
    ...uint16(0),
    ...uint32(faces.length),
    ...bases.flatMap((base) => uint32(base)),
    ...moved.flat(),
  ]);
}

/** Apple Color Emoji of macOS, a collection of 2 faces whose `sbix` table is 191 MB (US-052). */
export const APPLE_COLOR_EMOJI_PATH = "/System/Library/Fonts/Apple Color Emoji.ttc";

/**
 * The parts of a face joined in their order into its file, as `facePartsToFile` joins them but
 * written apart from it, to check it: small files only, its bytes spread.
 *
 * @param parts - the face's parts
 * @returns its file
 */
export function joinedParts(parts: readonly Readonly<Uint8Array>[]): Uint8Array {
  return Uint8Array.from(parts.flatMap((part) => [...part]));
}

/**
 * The SHA-256 of a face's parts joined in their order, read part after part, never joined.
 *
 * @param parts - the face's parts
 * @returns the digest, in lower-case hexadecimal
 */
export function partsDigest(parts: readonly Readonly<Uint8Array>[]): string {
  return parts.reduce((hash, part) => hash.update(part), createHash("sha256")).digest("hex");
}

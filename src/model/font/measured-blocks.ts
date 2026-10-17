import type { UnicodeBlock } from "./unicode-block";

/**
 * The Unicode blocks of the scripts the library measures without shaping (ADR-0052): Latin with
 * its extensions and the combining diacritical marks, Greek, Cyrillic and Hebrew, and the punctuation and symbols industrial labels
 * use (currency, letterlike symbols, arrows, mathematical operators). A font's metrics are read for
 * their characters (`REF-UNICODE-BLOCKS`).
 */
export const MEASURED_BLOCKS: readonly UnicodeBlock[] = [
  { first: 0x00_00, last: 0x00_7f, name: "Basic Latin" },
  { first: 0x00_80, last: 0x00_ff, name: "Latin-1 Supplement" },
  { first: 0x01_00, last: 0x01_7f, name: "Latin Extended-A" },
  { first: 0x01_80, last: 0x02_4f, name: "Latin Extended-B" },
  { first: 0x03_00, last: 0x03_6f, name: "Combining Diacritical Marks" },
  { first: 0x03_70, last: 0x03_ff, name: "Greek and Coptic" },
  { first: 0x04_00, last: 0x04_ff, name: "Cyrillic" },
  { first: 0x05_90, last: 0x05_ff, name: "Hebrew" },
  { first: 0x1e_00, last: 0x1e_ff, name: "Latin Extended Additional" },
  { first: 0x20_00, last: 0x20_6f, name: "General Punctuation" },
  { first: 0x20_a0, last: 0x20_cf, name: "Currency Symbols" },
  { first: 0x21_00, last: 0x21_4f, name: "Letterlike Symbols" },
  { first: 0x21_90, last: 0x21_ff, name: "Arrows" },
  { first: 0x22_00, last: 0x22_ff, name: "Mathematical Operators" },
];

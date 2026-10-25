import { integerQuotient, nearestInteger, nonNegativeRemainder } from "../../math";

/** A unit of size above a byte. */
type ByteUnit = {
  /** Its symbol, as the toolbar writes it. */
  readonly symbol: string;
  /** Its bytes. */
  readonly bytes: number;
};

/** How many times each unit holds the one below it, a byte the first, as SI prefixes multiply. */
const UNIT_STEP = 1000;

/** The tenths in a unit, the size written to one decimal. */
const TENTHS = 10;

/** kB, 1 000 bytes. */
const KILOBYTE: ByteUnit = { bytes: UNIT_STEP, symbol: "kB" };

/** MB, 1 000 kB. */
const MEGABYTE: ByteUnit = { bytes: UNIT_STEP * KILOBYTE.bytes, symbol: "MB" };

/** GB, 1 000 MB, the largest unit written. */
const GIGABYTE: ByteUnit = { bytes: UNIT_STEP * MEGABYTE.bytes, symbol: "GB" };

/** The units below the largest, the smallest first. */
const SMALLER_UNITS: readonly ByteUnit[] = [KILOBYTE, MEGABYTE];

/**
 * Converts a size into tenths of a unit, rounded half up (`nearestInteger`): 1 248 bytes is 12
 * tenths of a kB.
 *
 * @kind format
 * @param bytes - the size, a whole number of bytes
 * @param unit - kB, MB or GB
 * @returns the tenths, a whole number
 * @see RULE-interaction-7-navigation-and-history
 */
function bytesToTenths(bytes: number, unit: ByteUnit): number {
  return nearestInteger((bytes * TENTHS) / unit.bytes);
}

/**
 * Writes a size in bytes as the toolbar shows it: below 1 000, in bytes, `999 B`; above, in
 * the first of kB, MB and GB, each 1 000 times the one before, that keeps it below 1 000 once
 * rounded, GB the last, to one decimal rounded half up, a decimal of 0 left out, the units the
 * tenths' quotient by 10 (`integerQuotient`) and the decimal their remainder
 * (`nonNegativeRemainder`): 1 248 bytes is `1.2 kB`, 999 950 is `1 MB`, 100 000 000 is `100 MB`.
 *
 * @kind format
 * @param bytes - the size, a whole number of bytes, 0 or more
 * @returns its text, the number then the unit
 * @see RULE-interaction-7-navigation-and-history
 */
export function formatByteSize(bytes: number): string {
  if (bytes < UNIT_STEP) {
    return `${bytes} B`;
  }

  const unit =
    SMALLER_UNITS.find((each) => bytesToTenths(bytes, each) < UNIT_STEP * TENTHS) ?? GIGABYTE;
  const tenths = bytesToTenths(bytes, unit);
  const tenth = nonNegativeRemainder(tenths, TENTHS);

  return `${integerQuotient(tenths, TENTHS)}${tenth === 0 ? "" : `.${tenth}`} ${unit.symbol}`;
}

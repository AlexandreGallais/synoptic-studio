/**
 * The integers a text's numbers range over, both ends included (F05 refinement, round 2,
 * 2026-10-14): its size from 1 to 100 000 user units, its weight from 100 (thin) to 900 (black,
 * `REF-CSS-FONTS-4`), its opacity from 0 to 100 %; its letter and word spacing from −100 000 to
 * 100 000 user units, the size's bound (2026-10-15, provisional, to confirm at VAL-014).
 */
export const TEXT_RANGES: {
  /** Opacity, in percent. */
  readonly opacity: {
    /** Fully transparent. */
    readonly first: number;
    /** Fully opaque. */
    readonly last: number;
  };
  /** Size, the em, in user units. */
  readonly size: {
    /** Smallest size. */
    readonly first: number;
    /** Largest size. */
    readonly last: number;
  };
  /** Letter spacing and word spacing, in user units. */
  readonly spacing: {
    /** Tightest. */
    readonly first: number;
    /** Loosest. */
    readonly last: number;
  };
  /** Weight. */
  readonly weight: {
    /** Thin. */
    readonly first: number;
    /** Black. */
    readonly last: number;
  };
} = {
  opacity: { first: 0, last: 100 },
  size: { first: 1, last: 100_000 },
  spacing: { first: -100_000, last: 100_000 },
  weight: { first: 100, last: 900 },
};

/**
 * What is kept of a shape once it is first read (ADR-0096, ADR-0097): its key in the held counts
 * and the length of its JSON text, a shape never being changed once made (ADR-0014).
 */
export type ShapeMemo = {
  /** Its key in the held counts, an integer ≥ 0, the next of the keys given. */
  readonly key: number;
  /** The length of its JSON text, in 16-bit code units. */
  readonly length: number;
};

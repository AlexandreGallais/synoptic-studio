/**
 * A value computed once and kept with the inputs it was computed from, until they change.
 *
 * @template T - type of the value
 */
export type CachedValue<T> = {
  /** The plain inputs it was computed from, written as one string. */
  readonly key: string;
  /** The value computed. */
  readonly value: T;
};

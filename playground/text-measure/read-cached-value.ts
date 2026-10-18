import type { CachedValue } from "./cached-value";

/**
 * What a value kept is looked up by: the inputs it is computed from.
 *
 * @template S - type of the object it is computed with
 */
type CachedInput<S extends object> = {
  /** The object it is computed with, compared by identity: the families kept, or the metrics. */
  readonly source: S;
  /** The plain inputs, written as one string. */
  readonly key: string;
};

/**
 * Reads a value kept for the object it is computed with when it was computed from the same plain
 * inputs; otherwise computes it and keeps it in place of the one before (US-048).
 *
 * @kind procedure
 * @template S - type of the object it is computed with
 * @template T - type of the value
 * @param cache - the values kept, by that object
 * @param input - the object and the plain inputs it is computed from
 * @param compute - computes it from them
 * @returns the value kept, or computed now
 * @see docs/backlog/stories/E15-F17-US-048-fluid-changes-held-keys.md
 */
export function readCachedValue<S extends object, T>(
  cache: WeakMap<S, CachedValue<T>>,
  input: CachedInput<S>,
  compute: () => T,
): T {
  const cached = cache.get(input.source);

  if (cached?.key === input.key) {
    return cached.value;
  }

  const value = compute();

  cache.set(input.source, { key: input.key, value });

  return value;
}

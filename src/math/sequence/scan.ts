import { integerQuotient } from "../arithmetic";

import type { ScanStep } from "./scan-step";

/**
 * The states after each item of a run, and the last of them.
 *
 * @template S - type of the states
 */
type Scanned<S> = {
  /** The state after each item, in order. */
  readonly states: readonly S[];
  /** The state after the last item; the initial one for no item. */
  readonly last: S;
};

/** Runs at most this long are scanned one item after the other. */
const SHORT_RUN = 8;

/**
 * A short run scanned one item after the other, each state added to those before.
 *
 * @kind math
 * @template S - type of the states
 * @template T - type of the items
 * @param items - the run, at most `SHORT_RUN` long
 * @param step - the state after an item, from the state before it
 * @param initial - the state before the first item
 * @returns the states after each item, and the last
 * @see REF-WIKI-PREFIX-SUM
 */
function shortScanned<S, T>(items: readonly T[], step: ScanStep<S, T>, initial: S): Scanned<S> {
  return items.reduce<Scanned<S>>(
    (done, item) => {
      const state = step(done.last, item);

      return { last: state, states: [...done.states, state] };
    },
    { last: initial, states: [] },
  );
}

/**
 * A run scanned by halves: the second half starts from the last state of the first, so that each
 * state is copied once a halving, not once an item.
 *
 * @kind math
 * @template S - type of the states
 * @template T - type of the items
 * @param items - the run
 * @param step - the state after an item, from the state before it
 * @param initial - the state before the first item
 * @returns the states after each item, and the last
 * @see REF-WIKI-PREFIX-SUM
 */
function scanned<S, T>(items: readonly T[], step: ScanStep<S, T>, initial: S): Scanned<S> {
  const middle = integerQuotient(items.length, 2);
  const first =
    items.length <= SHORT_RUN ? undefined : scanned(items.slice(0, middle), step, initial);

  return first === undefined
    ? shortScanned(items, step, initial)
    : joinedScans(first, scanned(items.slice(middle), step, first.last));
}

/**
 * Two runs scanned one after the other, joined: the states of both, the last of the second.
 *
 * @kind math
 * @template S - type of the states
 * @param first - the first run scanned
 * @param second - the run after it, scanned from the first's last state
 * @returns the states of both, and the second's last
 * @see REF-WIKI-PREFIX-SUM
 */
function joinedScans<S>(first: Scanned<S>, second: Scanned<S>): Scanned<S> {
  return { last: second.last, states: [...first.states, ...second.states] };
}

/**
 * The scan of a sequence: the whole sequence of the states a fold goes through, the state after
 * each item computed from the state before it (`REF-WIKI-PREFIX-SUM`, "the scan returns the whole
 * sequence of results"). The sequence is scanned by halves, so that the states are copied about
 * `n log₂ n` times rather than `n²`, the depth of the calls `log₂ n`.
 *
 * Formula: s₀ = initial; sᵢ = step(sᵢ₋₁, xᵢ) for i = 1 … n; gives s₁ … sₙ.
 *
 * @kind math
 * @template S - type of the states
 * @template T - type of the items
 * @param items - the sequence
 * @param step - the state after an item, from the state before it
 * @param initial - the state before the first item
 * @returns the state after each item, in order; none for no item
 * @see REF-WIKI-PREFIX-SUM
 */
export function scan<S, T>(items: readonly T[], step: ScanStep<S, T>, initial: S): readonly S[] {
  return scanned(items, step, initial).states;
}

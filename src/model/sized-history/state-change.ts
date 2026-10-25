import type { HistoryState } from "../history";

/**
 * A state joining or leaving the past or the states to come: the state, and by how many
 * states each of its shapes is held after it, 1 more as it joins, 1 less as it leaves (ADR-0097).
 *
 * @template T - type of the shapes, or of what is kept of each
 */
export type StateChange<T> = {
  /** 1 when the state joins, −1 when it leaves. */
  readonly by: 1 | -1;
  /** The state added or removed. */
  readonly state: HistoryState<T>;
};

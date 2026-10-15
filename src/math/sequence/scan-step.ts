/**
 * One step of a scan: the state after an item, from the state before it.
 *
 * @template S - type of the states
 * @template T - type of the items
 */
export type ScanStep<S, T> = (state: S, item: T) => S;

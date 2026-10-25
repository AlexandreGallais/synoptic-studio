/**
 * A state of the drawing a history holds: the shapes and the selection, as they were when
 * the state was recorded, the selection replaced while it changes alone (ADR-0095).
 *
 * @template T - type of the shapes, or of what is kept of each
 */
export type HistoryState<T> = {
  /** The shapes in the order drawn, the last on top, those unchanged shared with the next state. */
  readonly drawing: readonly T[];
  /** The indices of the shapes selected, in the order drawn. */
  readonly selection: readonly number[];
};

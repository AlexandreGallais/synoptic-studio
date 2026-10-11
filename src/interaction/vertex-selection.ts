/** Vertices selected in Points mode, and where a range starts (`docs/domain/interaction.md` §3). */
export type VertexSelection = {
  /** Numbers of the selected vertices, from 0 in contour order (Q11), ascending, no repeat. */
  readonly selected: readonly number[];
  /** Vertex last clicked without Shift, where a Shift range starts; `undefined` before any. */
  readonly start: number | undefined;
};

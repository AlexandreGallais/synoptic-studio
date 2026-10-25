import type { VertexSelection } from "./vertex-selection";

/**
 * Selection after a plain click on a vertex, on its handle or its list row: that vertex alone,
 * and the start of the next range (F03.AC5).
 *
 * @kind domain
 * @param vertex - number of the vertex clicked, from 0
 * @returns the new selection
 * @see RULE-interaction-3-node-editing
 */
export function selectionAfterClick(vertex: number): VertexSelection {
  return { selected: [vertex], start: vertex };
}

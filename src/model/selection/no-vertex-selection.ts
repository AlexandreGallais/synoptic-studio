import type { VertexSelection } from "./vertex-selection";

/** No vertex selected and no range start: the selection after it is cleared (F03.AC5). */
export const NO_VERTEX_SELECTION: VertexSelection = { selected: [], start: undefined };

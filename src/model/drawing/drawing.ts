import type { DrawingShape } from "./drawing-shape";

/** A drawing: its shapes in the order drawn, the last on top, flat until E27 makes it a tree (F35). */
export type Drawing = readonly DrawingShape[];

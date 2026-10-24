import type { Shape } from "../shape";
import type { Text } from "../text";

/** A shape of a drawing: a shape a tool makes (rectangle, polygon, round shape) or a text (F35). */
export type DrawingShape = Shape | Text;

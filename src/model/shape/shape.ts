import type { FreeShape } from "../free-shape";
import type { RegularPolygon } from "../regular-polygon";
import type { RoundShape } from "../round-shape";

/** A shape of the model: a free shape (the rectangle tool's too, ADR-0032), a regular polygon or a round shape. */
export type Shape = FreeShape | RegularPolygon | RoundShape;

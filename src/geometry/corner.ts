import type { Point } from "../math";

/** Vertex of a shape's contour with the corner radius requested at it (glossary: corner radius). */
export type Corner = {
  /** Vertex: integer coordinates in the model, fractional when derived (a polygon, ADR-0003). */
  readonly point: Point;
  /** Requested corner radius, integer >= 0; the effective radius is derived (Q8). */
  readonly radius: number;
};

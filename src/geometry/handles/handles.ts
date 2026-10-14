import type { HandleMode } from "./handle-mode";
import type { Vector } from "../../math";

/** The two handles of a point, vectors from it (`REF-FIGMA-VECTOR-NETWORK`, ADR-0046). */
export type Handles = {
  /** Whether the point is in handles mode; off, its corner radius is drawn instead, both kept. */
  readonly isOn: boolean;
  /** How typing one handle moves the other. */
  readonly mode: HandleMode;
  /** Handle towards the previous point, a vector from the point; (0, 0) leaves that side straight. */
  readonly before: Vector;
  /** Handle towards the next point, a vector from the point; (0, 0) leaves that side straight. */
  readonly after: Vector;
};

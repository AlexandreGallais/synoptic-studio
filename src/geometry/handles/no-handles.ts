import type { Handles } from "./handles";

/** Handles of a new corner: off, in mode angle and length, both (0, 0) — the drawing unchanged (EN-024, `REF-FIGMA-VECTOR-NETWORK`). */
export const NO_HANDLES: Handles = {
  after: { x: 0, y: 0 },
  before: { x: 0, y: 0 },
  isOn: false,
  mode: "angle-and-length",
};

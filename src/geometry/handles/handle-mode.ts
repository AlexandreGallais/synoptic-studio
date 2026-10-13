import type { HANDLE_MODES } from "./handle-modes";

/**
 * How a point's two handles follow each other (`REF-FIGMA-HANDLE-MIRRORING`): `none`, each free;
 * `angle`, on one line through the point, each its own length; `angle-and-length`, on one line
 * and of the same length; `auto`, computed from the neighbouring points by a tension
 * (`DERIV-handle-tension`).
 */
export type HandleMode = (typeof HANDLE_MODES)[number];

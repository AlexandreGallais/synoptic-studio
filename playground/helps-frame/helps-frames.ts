import type { HelpsFrame } from "./helps-frame";

/**
 * The frame the helps of the layer above the drawing are computed in, by the page's document
 * (SP-030's answer 7): none until the layer is first drawn.
 */
export const HELPS_FRAMES = new WeakMap<Document, HelpsFrame>();

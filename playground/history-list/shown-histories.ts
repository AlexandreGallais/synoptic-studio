import type { SizedHistory } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * The history the list of steps of each page shows (US-077), by its document, so that a redraw
 * leaving the history as it was writes no row again.
 */
export const SHOWN_HISTORIES = new WeakMap<Document, SizedHistory<KeptShape>>();

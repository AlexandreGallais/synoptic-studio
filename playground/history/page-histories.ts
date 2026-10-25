import type { SizedHistory } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * The history of the drawing of each page, by its document: the shapes as the page keeps
 * them and the selection, with its size kept (`SizedHistory`, ADR-0097), in a module's constant, as `KEPT_TEXT_METRICS` (ADR-0095, decision 6);
 * none until the page is first redrawn, or once it is emptied by Reset.
 */
export const PAGE_HISTORIES = new WeakMap<Document, SizedHistory<KeptShape>>();

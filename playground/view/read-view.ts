import { OPENING_VIEW } from "./opening-view";
import { PAGE_VIEWS } from "./page-views";

import type { View } from "../../src";

/**
 * Reads the view of the drawing (ADR-0098): the one kept for the page (`PAGE_VIEWS`), or the
 * opening view.
 *
 * @kind procedure
 * @param document - page document
 * @returns its scale and origin
 */
export function readView(document: Document): View {
  return PAGE_VIEWS.get(document) ?? OPENING_VIEW;
}

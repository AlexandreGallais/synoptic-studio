import type { View } from "../../src";

/**
 * The view of the drawing, by the page's document (ADR-0098): kept outside the drawing, its
 * history and its export; none until the view first changes, nor once Reset or a step of the guided
 * test brings back the opening view.
 */
export const PAGE_VIEWS = new WeakMap<Document, View>();

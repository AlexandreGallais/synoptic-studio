import { wheelZoomFactor, zoomedView } from "../../src";

import { PAGE_VIEWS } from "./page-views";
import { readView } from "./read-view";
import { WHEEL_ZOOMS } from "./wheel-zooms";

/**
 * Zooms the view by the wheel's and the pinch's deltas gathered since the last frame, about the
 * pointer's last offset (`zoomedView`, `wheelZoomFactor`, SP-030's answer 4), then forgets them;
 * nothing when none were asked.
 *
 * @kind procedure
 * @param document - page document
 */
export function applyWheelZoom(document: Document): void {
  const zoom = WHEEL_ZOOMS.get(document);

  if (zoom === undefined) {
    return;
  }

  const view = readView(document);

  WHEEL_ZOOMS.delete(document);
  PAGE_VIEWS.set(document, zoomedView(view, zoom.offset, view.scale * wheelZoomFactor(zoom.delta)));
}

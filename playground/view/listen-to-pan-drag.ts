import { pannedView, subtract } from "../../src";
import { readPointerOffset, selectElement } from "../page";

import { HELD_SPACES } from "./held-spaces";
import { PAGE_VIEWS } from "./page-views";
import { PAN_DRAGS } from "./pan-drags";
import { POINTER_OFFSETS } from "./pointer-offsets";
import { readView } from "./read-view";

/**
 * Starts a drag that pans, pressed with the middle button, or with the main one while `Space` is
 * held (SP-030's answer 2): its default prevented, the press kept from the other listeners of the
 * canvas, which it takes so that the drag goes on beyond its edges; a pan released before, without
 * its click, dropped first.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the press on the canvas
 * @see RULE-interaction-7-navigation-and-history
 */
function startPanDrag(document: Document, event: PointerEvent): void {
  PAN_DRAGS.delete(document);

  // The middle button pans, and the main one while Space is held.
  const buttons = HELD_SPACES.has(document) ? [0, 1] : [1];

  if (!buttons.includes(event.button)) {
    return;
  }

  event.preventDefault();
  event.stopImmediatePropagation();
  selectElement(document, "canvas").setPointerCapture(event.pointerId);
  PAN_DRAGS.set(document, { client: { x: event.clientX, y: event.clientY }, isReleased: false });
}

/**
 * Pans the view by a move of the pointer during a drag that pans: by its opposite, the drawing
 * following the pointer (`pannedView`), the move kept from the other listeners, so that it keeps the
 * pointer's offset itself (`POINTER_OFFSETS`), a key then zooming about it; nothing without one.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the pointer's move on the canvas
 * @param update - redraws the page
 * @see RULE-interaction-7-navigation-and-history
 */
function movePanDrag(document: Document, event: PointerEvent, update: () => void): void {
  const drag = PAN_DRAGS.get(document);

  if (drag === undefined || drag.isReleased) {
    return;
  }

  const client = { x: event.clientX, y: event.clientY };

  event.stopImmediatePropagation();
  POINTER_OFFSETS.set(document, readPointerOffset(selectElement(document, "canvas"), event).point);
  PAGE_VIEWS.set(document, pannedView(readView(document), subtract(drag.client, client)));
  PAN_DRAGS.set(document, { client, isReleased: false });
  update();
}

/**
 * Ends a drag that pans at its release, or as the browser cancels it, kept from the other
 * listeners; the drag kept, released, until the click that follows it, which it keeps from them
 * too, so that nothing is picked nor placed.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the release, the drag cancelled, or the click after it
 * @see RULE-interaction-7-navigation-and-history
 */
function stopPanDrag(document: Document, event: Event): void {
  const drag = PAN_DRAGS.get(document);

  if (drag === undefined) {
    return;
  }

  event.stopImmediatePropagation();

  if (event.type === "click") {
    PAN_DRAGS.delete(document);
  } else {
    PAN_DRAGS.set(document, { ...drag, isReleased: true });
  }
}

/**
 * Listens to a drag that pans the view (SP-030's answer 2): pressed with `Space` held or with the
 * middle button, the canvas takes the pointer (`startPanDrag`), each move pans by its opposite
 * (`movePanDrag`), the drawing following the pointer, and no other listener of the canvas hears the
 * press, its moves, its release nor the click after it (`stopPanDrag`), so that nothing is selected,
 * moved nor placed. Wired before the drawing's listeners.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see RULE-interaction-7-navigation-and-history
 */
export function listenToPanDrag(document: Document, update: () => void): void {
  const canvas = selectElement(document, "canvas");

  canvas.addEventListener("pointerdown", (event) => {
    startPanDrag(document, event);
  });

  canvas.addEventListener("pointermove", (event) => {
    movePanDrag(document, event, update);
  });

  for (const type of ["pointerup", "pointercancel", "click"]) {
    canvas.addEventListener(type, (event) => {
      stopPanDrag(document, event);
    });
  }
}

import { LISTENER_CONTROLLERS } from "./listener-controllers";

/**
 * Reads the signal that the listeners of the page put on its document and its window are given,
 * so that the next mount removes them (`REF-MDN-ADD-EVENT-LISTENER`, `signal`); a document without
 * a controller is given a new one.
 *
 * @kind procedure
 * @param document - page document
 * @returns the signal of the document's controller
 */
export function readListenerSignal(document: Document): AbortSignal {
  const kept = LISTENER_CONTROLLERS.get(document);

  if (kept !== undefined) {
    return kept.signal;
  }

  const controller = new AbortController();

  LISTENER_CONTROLLERS.set(document, controller);

  return controller.signal;
}

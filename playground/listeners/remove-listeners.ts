import { LISTENER_CONTROLLERS } from "./listener-controllers";

/**
 * Removes the listeners a mount before this one put on the document and its window: their
 * controller aborted, then dropped, so that the listeners of the mount to come get a new one
 * (`readListenerSignal`); a document mounted once, as in the browser, has none to remove. The
 * listeners on the page's elements go with the elements a test loads again.
 *
 * @kind procedure
 * @param document - page document
 */
export function removeListeners(document: Document): void {
  LISTENER_CONTROLLERS.get(document)?.abort();
  LISTENER_CONTROLLERS.delete(document);
}

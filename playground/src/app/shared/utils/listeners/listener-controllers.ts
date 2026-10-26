/** The controller of the listeners a mount put on a document and its window, by their document: aborted at the next mount, it removes them (`REF-MDN-ADD-EVENT-LISTENER`). */
export const LISTENER_CONTROLLERS = new WeakMap<Document, AbortController>();

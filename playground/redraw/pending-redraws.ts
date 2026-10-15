/** The pages waiting for their next animation frame to be redrawn, by their document (US-048): a page is redrawn once at that frame, the requests before it gathered. */
export const PENDING_REDRAWS = new WeakSet<Document>();

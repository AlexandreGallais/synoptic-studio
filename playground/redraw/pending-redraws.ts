/** The redraws asked and not drawn yet, by their page's document (US-048): each drawn once, at the next animation frame or earlier, before a click acts (`drawPendingRedraw`), the requests before it gathered. */
export const PENDING_REDRAWS = new WeakMap<Document, () => void>();

/**
 * The pages whose last action finished and waits for its step, by their document: the step
 * is taken once the page is redrawn from the action, the shape the panel edits then written back
 * as last drawn (ADR-0095, decision 3; `scheduleRedraw`).
 */
export const DUE_HISTORY_STEPS = new WeakSet<Document>();

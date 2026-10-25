/** The asynchronous tasks a page started from its event listeners, by its document: font files and installed fonts read; kept, never floating, so that a test awaits them. */
export const PENDING_TASKS = new WeakMap<Document, readonly Promise<void>[]>();

/**
 * The arrows held outside the panel's fields, by their document, in the order pressed (US-081):
 * each arrow's key nudges by their sum, the browser repeating only the last pressed, and their
 * nudges are one step of the history from the first press to the last release.
 */
export const HELD_ARROWS = new WeakMap<Document, readonly string[]>();

/**
 * The tool armed at the last press on the drawing, by the page's document: a click picks a
 * shape only when its press was made with the selector, so that the click ending a drag or placing
 * a shape with a tool never picks one too.
 */
export const PRESSED_TOOLS = new WeakMap<Document, string>();

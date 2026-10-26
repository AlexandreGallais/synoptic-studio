import type { Point } from "editor";

/**
 * The point pressed with a tool armed, by the page's document, once its release is a click, the
 * pointer having stayed within 3 pixels of it (SP-030's answer 8): the click places its shape
 * centred on it, not on the release, at any zoom; dropped by that click or the next press.
 */
export const CLICKED_PRESSES = new WeakMap<Document, Point>();

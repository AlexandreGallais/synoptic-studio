/**
 * The text each field acting as a number field showed before a text was typed in it,
 * read as it gets the focus or a key while no text typed waits in it: the value applied, from
 * which a relative value typed starts, which a value refused puts back and which the page reads
 * while a text typed is not applied (`readShownText`).
 */
export const SHOWN_TEXTS = new WeakMap<Element, string>();

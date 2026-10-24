/**
 * The number each field acting as a number field of its own showed last before a value was typed
 * in it (US-087), from which a relative value typed starts, and which a value refused puts back.
 */
export const SHOWN_NUMBERS = new WeakMap<Element, number>();

import type { FontSession } from "./font-session";

/** The font session of each page, by its font field: a page loaded again starts with Inter alone. */
export const FONT_SESSIONS = new WeakMap<Element, FontSession>();

import type { FontSession } from "./font-session";

/** The session whose families the font list shows, by the list (US-048): the list is written again only when the families kept or the installed fonts change. */
export const WRITTEN_FONT_LISTS = new WeakMap<Element, FontSession>();

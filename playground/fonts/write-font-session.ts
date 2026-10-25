import { selectElement } from "../page";

import { FONT_SESSIONS } from "./font-sessions";

import type { FontSession } from "./font-session";

/**
 * Keeps the page's fonts for the session.
 *
 * @kind procedure
 * @param document - page document
 * @param session - the families kept and the installed fonts listed
 * @see US-036
 */
export function writeFontSession(document: Document, session: FontSession): void {
  FONT_SESSIONS.set(selectElement(document, "text-font"), session);
}

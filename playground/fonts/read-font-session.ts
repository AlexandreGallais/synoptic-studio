import { DEFAULT_FONT } from "../../src";
import { selectElement } from "../page";

import { FONT_SESSIONS } from "./font-sessions";

import type { FontSession } from "./font-session";

/**
 * Reads the page's fonts: the families kept with their metrics, the installed fonts listed and
 * the family the text is measured with; Inter alone, the default font, before any is imported or
 * listed.
 *
 * @kind procedure
 * @param document - page document
 * @returns the session's fonts
 */
export function readFontSession(document: Document): FontSession {
  return (
    FONT_SESSIONS.get(selectElement(document, "text-font")) ?? {
      installed: [],
      isReading: false,
      kept: [DEFAULT_FONT],
      measured: DEFAULT_FONT.name,
      readAhead: [],
    }
  );
}

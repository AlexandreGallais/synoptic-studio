import { DEFAULT_FONT } from "../../src";
import { selectElement } from "../page";

import { FONT_SESSIONS } from "./font-sessions";

import type { FontSession } from "./font-session";

/**
 * Reads the page's fonts: the families kept with their metrics and the installed fonts listed;
 * Inter alone, the default font, before any is imported or listed.
 *
 * @kind procedure
 * @param document - page document
 * @returns the session's fonts
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function readFontSession(document: Document): FontSession {
  return (
    FONT_SESSIONS.get(selectElement(document, "text-font")) ?? {
      installed: [],
      kept: [DEFAULT_FONT],
    }
  );
}

import { selectElement } from "../page";

import { TOOLS } from "./tools";

/**
 * Reads the tool armed in the toolbar, the one whose button is pressed (F35).
 *
 * @kind procedure
 * @param document - page document
 * @returns `selector`, `rectangle`, `polygon`, `round` or `text`; the selector when none is pressed
 * @see US-063
 */
export function readArmedTool(document: Document): string {
  const pressed = TOOLS.find(
    ([id]) => selectElement(document, id).getAttribute("aria-pressed") === "true",
  );

  return pressed?.[1] ?? "selector";
}

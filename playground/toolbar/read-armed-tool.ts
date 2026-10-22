import { selectElement } from "../page";

import { TOOLS } from "./tools";

/**
 * Reads the tool armed in the toolbar, the one whose button is pressed (F35).
 *
 * @kind procedure
 * @param document - page document
 * @returns `selector`, `rectangle`, `polygon`, `round` or `text`; the selector when none is pressed
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function readArmedTool(document: Document): string {
  const pressed = TOOLS.find(
    ([id]) => selectElement(document, id).getAttribute("aria-pressed") === "true",
  );

  return pressed?.[1] ?? "selector";
}

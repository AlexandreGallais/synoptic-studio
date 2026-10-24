import { selectElement } from "../page";

import { TOOLS } from "./tools";

/**
 * Arms a tool of the toolbar, its button alone pressed: the selector, or the tool a click on the
 * drawing places a shape with (F35).
 *
 * @kind procedure
 * @param document - page document
 * @param tool - `selector`, `rectangle`, `polygon`, `round` or `text`
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function writeArmedTool(document: Document, tool: string): void {
  for (const [id, kind] of TOOLS) {
    selectElement(document, id).setAttribute("aria-pressed", String(kind === tool));
  }
}

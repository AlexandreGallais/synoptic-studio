import { selectElement } from "../page";

import { TOOLS } from "./tools";
import { writeArmedTool } from "./write-armed-tool";
import { writeDefaultShape } from "./write-default-shape";

/**
 * Listens to the toolbar: a tool's button arms it, the shape the panel edits left as it is, as in
 * Figma (F35); Reset puts back the shape the page opens with, alone, in Shape mode, the selector
 * armed; Pipeline shows or hides the pipeline column.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function listenToToolbar(document: Document): void {
  for (const [id, tool] of TOOLS) {
    selectElement(document, id).addEventListener("click", () => {
      writeArmedTool(document, tool);
    });
  }

  selectElement(document, "tool-output").addEventListener("click", () => {
    const isHidden = selectElement(document, "output").toggleAttribute("hidden");

    selectElement(document, "tool-output").setAttribute("aria-pressed", String(!isHidden));
  });

  selectElement(document, "tool-reset").addEventListener("click", () => {
    writeDefaultShape(document);
    writeArmedTool(document, "selector");
    selectElement(document, "mode-shape").click();
  });
}

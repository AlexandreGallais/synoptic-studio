import { selectElement } from "./select-element";
import { writeDefaultShape } from "./write-default-shape";
import { writeWholeShapeFields } from "./write-whole-shape-fields";

/** Toolbar buttons and the shape kind each one chooses. */
const TOOLS = [
  ["tool-rectangle", "rectangle"],
  ["tool-polygon", "polygon"],
] as const;

/**
 * Chooses a shape kind in the selector, its fields made whole first, and fires the selector's
 * `change` event, as a choice in the selector would.
 *
 * @kind procedure
 * @param document - page document
 * @param kind - `rectangle` or `polygon`
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
function writeShapeChoice(document: Document, kind: string): void {
  const select = selectElement(document, "shape");

  if (!(select instanceof HTMLSelectElement)) {
    return;
  }

  writeWholeShapeFields(document);
  select.value = kind;
  select.dispatchEvent(new Event("change"));
}

/**
 * Listens to the toolbar: a shape button chooses its kind, as the shape selector would; Reset puts
 * back the shape the page opens with, in Shape mode; Pipeline shows or hides the pipeline column.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
export function listenToToolbar(document: Document): void {
  for (const [id, kind] of TOOLS) {
    selectElement(document, id).addEventListener("click", () => {
      writeShapeChoice(document, kind);
    });
  }

  selectElement(document, "tool-output").addEventListener("click", () => {
    const isHidden = selectElement(document, "output").toggleAttribute("hidden");

    selectElement(document, "tool-output").setAttribute("aria-pressed", String(!isHidden));
  });

  selectElement(document, "tool-reset").addEventListener("click", () => {
    writeDefaultShape(document);
    selectElement(document, "mode-shape").click();
  });
}

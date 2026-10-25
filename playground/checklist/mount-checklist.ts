import { CHECKLISTS } from "../checklists";
import { selectElement, writeText } from "../page";

import { readTickedLines } from "./read-ticked-lines";
import { writeChecklistLines } from "./write-checklist-lines";
import { writeTickedLines } from "./write-ticked-lines";

/**
 * Mounts the checklist the page address asks (`?checklist=E15`): its lines with their boxes, those
 * ticked in this browser ticked again, and each tick kept. Hidden when the address asks none, or
 * a checklist that does not exist.
 *
 * @kind procedure
 * @param document - the playground's document
 */
export function mountChecklist(document: Document): void {
  const query = new URLSearchParams(document.location.search);
  const name = query.get("checklist") ?? "";
  const lines = CHECKLISTS.get(name) ?? [];
  const section = selectElement(document, "checklist");
  const list = selectElement(document, "checklist-lines");

  section.hidden = lines.length === 0;
  writeText(document, "checklist-name", name);
  writeChecklistLines(document, lines, readTickedLines(document, name));

  list.addEventListener("change", () => {
    const boxes = [...list.querySelectorAll("input")];

    writeTickedLines(
      document,
      name,
      boxes.filter((box) => box.checked).map((box) => box.value),
    );
  });
}

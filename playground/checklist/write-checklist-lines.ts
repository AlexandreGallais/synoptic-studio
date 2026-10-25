import { selectElement } from "../page";

import type { ChecklistLine } from "../checklists";

/**
 * Writes one line of a checklist: its check box, its feature, what to try and what to see.
 *
 * @kind procedure
 * @param document - the playground's document
 * @param line - its feature, what to try and what to see
 * @param isTicked - whether its box starts ticked
 * @returns the line's list item
 * @see US-051
 */
function createLineElement(
  document: Document,
  line: ChecklistLine,
  isTicked: boolean,
): HTMLElement {
  const item = document.createElement("li");
  const label = document.createElement("label");
  const box = document.createElement("input");
  const parts = [
    ["checklist-feature", line.feature],
    ["checklist-todo", line.todo],
    ["checklist-see", line.see],
  ].map(([name = "", text = ""]) => {
    const part = document.createElement("span");

    part.className = name;
    part.textContent = text;

    return part;
  });

  box.type = "checkbox";
  box.checked = isTicked;
  box.value = line.todo;
  label.append(box, ...parts);
  item.append(label);

  return item;
}

/**
 * Writes the lines of a checklist into the page, each box ticked when its line is among those kept.
 *
 * @kind procedure
 * @param document - the playground's document
 * @param lines - each line's feature, what to try and what to see, in order
 * @param tickedTodos - what the ticked lines say to try
 * @see US-051
 */
export function writeChecklistLines(
  document: Document,
  lines: readonly ChecklistLine[],
  tickedTodos: ReadonlySet<string>,
): void {
  selectElement(document, "checklist-lines").replaceChildren(
    ...lines.map((line) => createLineElement(document, line, tickedTodos.has(line.todo))),
  );
}

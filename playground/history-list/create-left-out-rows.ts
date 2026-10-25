/**
 * Creates the line of the steps the list leaves out on one side, their count with its
 * digits grouped by commas, `6 earlier steps`, `1 later step`; none left out, no line.
 *
 * @kind procedure
 * @param document - page document
 * @param count - how many states are left out on that side
 * @param side - `earlier` before the rows, `later` after them
 * @returns the line, or none
 * @see RULE-interaction-7-navigation-and-history
 */
export function createLeftOutRows(
  document: Document,
  count: number,
  side: "earlier" | "later",
): readonly HTMLLIElement[] {
  if (count === 0) {
    return [];
  }

  const item = document.createElement("li");

  item.className = "history-left-out";
  item.textContent = `${count.toLocaleString("en-US")} ${side} ${count === 1 ? "step" : "steps"}`;

  return [item];
}

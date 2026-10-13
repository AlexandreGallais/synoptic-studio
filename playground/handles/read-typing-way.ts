import { selectElement } from "../page";

/**
 * How the handle fields are typed, read from the list: as x and y on the page (`page`), or as dx
 * and dy from the point (`point`).
 *
 * @kind procedure
 * @param document - page document
 * @returns `point` when dx and dy are chosen, `page` otherwise
 * @see docs/backlog/stories/E14-F12-US-023-handles-relative-to-point.md
 */
export function readTypingWay(document: Document): "page" | "point" {
  const list = selectElement(document, "handle-method");

  return list instanceof HTMLSelectElement && list.value === "point" ? "point" : "page";
}

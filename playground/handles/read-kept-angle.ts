import { selectElement } from "../page";

/**
 * The angle typed for a handle of length 0 (`writeKeptAngle`), when it was typed for the same
 * point and way; 0 otherwise.
 *
 * @kind procedure
 * @param document - page document
 * @param end - which handle
 * @param owner - the point's number and the way chosen, as `2 side`
 * @returns the angle kept, or 0
 * @see docs/backlog/stories/E14-F12-US-024-handles-as-angle-and-length.md
 */
export function readKeptAngle(document: Document, end: "after" | "before", owner: string): number {
  const [kept = "", angle] = (
    selectElement(document, `handle-${end}-x`).dataset["keptAngle"] ?? ""
  ).split("|", 2);

  return kept === owner ? Number(angle) : 0;
}

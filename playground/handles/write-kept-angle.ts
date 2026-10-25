import { selectElement } from "../page";

/**
 * Keeps an angle typed for a handle of length 0, which has no angle of its own, in its field's
 * data, with its owner — the point's number and the way chosen — so that another point or way
 * never reads it (`readKeptAngle`).
 *
 * @kind procedure
 * @param document - page document
 * @param end - which handle
 * @param keptAngle - the point's number, then "|", then the angle typed, as `2 side|330`
 */
export function writeKeptAngle(
  document: Document,
  end: "after" | "before",
  keptAngle: string,
): void {
  selectElement(document, `handle-${end}-x`).dataset["keptAngle"] = keptAngle;
}

import { selectElement, writeInputValue } from "../page";

/** Fields of the shape that take whole numbers once a rectangle or a polygon is chosen. */
const WHOLE_FIELDS = ["width", "height", "shape-x", "shape-y"];

/**
 * Rounds the number of a field to a whole number, an empty field left empty.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of a number input
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
function writeRoundedField(document: Document, id: string): void {
  const field = selectElement(document, id);

  if (field instanceof HTMLInputElement && field.value !== "") {
    writeInputValue(document, id, Math.round(Number(field.value)));
  }
}

/**
 * Reads the first radius kept for the points on the radius field.
 *
 * @kind procedure
 * @param radius - the radius field
 * @returns that radius, 0 when none is kept
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
function readFirstKeptRadius(radius: HTMLElement): number {
  const [first = "0"] = (radius.dataset["radii"] ?? "0").split(" ", 1);

  return Number(first);
}

/**
 * Gives every corner one radius: the radius field when it holds one, else the first radius kept
 * for the points (rounded), else 0; the radii of the points are dropped.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
function writeOneRadius(document: Document): void {
  const radius = selectElement(document, "radius");
  const first = readFirstKeptRadius(radius);

  delete radius.dataset["radii"];

  if (radius instanceof HTMLInputElement && radius.value === "") {
    writeInputValue(document, "radius", Math.round(first));
  }
}

/**
 * Makes the fields valid for the rectangle or the polygon chosen after a free shape: its width,
 * height and position rounded to whole numbers, and one radius for every corner instead of the
 * radii of the free shape's points.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-RET-001-retrospective-e01.md
 */
export function writeWholeShapeFields(document: Document): void {
  for (const id of WHOLE_FIELDS) {
    writeRoundedField(document, id);
  }

  writeOneRadius(document);
  delete selectElement(document, "shape").dataset["x"];
  delete selectElement(document, "shape").dataset["y"];
}

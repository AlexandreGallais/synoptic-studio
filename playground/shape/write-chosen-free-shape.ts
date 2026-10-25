import { readStoredFreeShape } from "./read-stored-free-shape";
import { writeStoredFreeShape } from "./write-stored-free-shape";

/**
 * Brings back the free shape kept when "Free shape" is chosen again in the selector: its own
 * width, height and radii, not those of the shape shown before.
 *
 * @kind procedure
 * @param document - page document
 */
export function writeChosenFreeShape(document: Document): void {
  const kept = readStoredFreeShape(document);

  if (kept !== undefined) {
    writeStoredFreeShape(document, kept);
  }
}

import { degreesInTurn } from "editor";

import { readNumber } from "../page";

/**
 * Reads the angle typed in the angle field, kept from 0 to 359: 370 reads 10, −90 reads
 * 270; an empty or fractional field gives a value the shape's check refuses.
 *
 * @kind procedure
 * @param document - page document
 * @returns the angle kept, in degrees, clockwise on screen
 */
export function readAngle(document: Document): number {
  return degreesInTurn(readNumber(document, "angle"));
}

import type { Size } from "./size";
import type { Vector } from "../../math";

/**
 * Half the size of a box: the vector from its top-left corner to its centre (`DERIV-turned-shape`,
 * k = (w/2, h/2)).
 *
 * Formula: (w/2, h/2).
 *
 * @kind geometry
 * @param size - width and height
 * @returns the half size, as a vector
 * @see DERIV-turned-shape
 */
export function halfSize(size: Size): Vector {
  return { x: size.width / 2, y: size.height / 2 };
}

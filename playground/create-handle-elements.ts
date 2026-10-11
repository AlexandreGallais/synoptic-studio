import { createPathElement, formatSvgNumber } from "../src";

import type { Point } from "../src";

/** Radius of a handle, in pixels: large enough to be clicked easily (F03). */
const HANDLE_RADIUS = 8;

/**
 * Path data of a full circle: two half circles, as an SVG path has no circle command
 * (`REF-SVG2-PATHS` §9.3.8).
 *
 * @kind procedure
 * @param center - point the handle is drawn on
 * @param radius - size of the handle in pixels, > 0
 * @returns a closed `d` made of two arcs
 * @see REF-SVG2-PATHS
 */
function writeCirclePathData(center: Point, radius: number): string {
  const left = `${formatSvgNumber(center.x - radius)} ${formatSvgNumber(center.y)}`;
  const right = `${formatSvgNumber(center.x + radius)} ${formatSvgNumber(center.y)}`;
  const arc = `A${String(radius)} ${String(radius)} 0 1 1`;

  return `M${left} ${arc} ${right} ${arc} ${left} Z`;
}

/**
 * Draws a handle on each vertex of the shape for Points mode: vertex 1 of the contour red,
 * vertex 2 green, so that the start and the direction of the contour can be read, the others
 * neutral. Vertex 1 is drawn last, on top when handles coincide (F03.AC4).
 *
 * @kind procedure
 * @param document - page document
 * @param vertices - vertices of the contour, in drawing order (Q11)
 * @returns one `<path>` per vertex, vertex 1 last
 * @see docs/backlog/stories/E01-F03-US-012-points-mode.md
 */
export function createHandleElements(
  document: Document,
  vertices: readonly Point[],
): readonly SVGPathElement[] {
  const handles = vertices.map((vertex, index) => {
    const handle = createPathElement(document, writeCirclePathData(vertex, HANDLE_RADIUS));

    handle.classList.add("handle");
    handle.dataset["vertex"] = String(index + 1);

    return handle;
  });

  handles[0]?.classList.add("first");
  handles[1]?.classList.add("second");

  return handles.toReversed();
}

import { createPathElement, formatCoordinatePair, formatSvgNumber, screenToUser } from "editor";

import { readView } from "../view";

import type { Point } from "editor";

/** Radius of a handle, in screen pixels: large enough to be clicked easily. */
const HANDLE_RADIUS = 8;

/** Radius of the ring drawn behind a selected handle, in screen pixels. */
const HALO_RADIUS = 13;

/**
 * Path data of a full circle: two half circles, as an SVG path has no circle command
 * (`REF-SVG2-PATHS` §9.3.8).
 *
 * @kind procedure
 * @param center - point the handle is drawn on
 * @param radius - size of the handle in user units, > 0
 * @returns a closed `d` made of two arcs
 */
function createCirclePathData(center: Point, radius: number): string {
  const left = formatCoordinatePair({ x: center.x - radius, y: center.y });
  const right = formatCoordinatePair({ x: center.x + radius, y: center.y });
  const arc = `A${formatSvgNumber(radius)} ${formatSvgNumber(radius)} 0 1 1`;

  return `M${left} ${arc} ${right} ${arc} ${left} Z`;
}

/**
 * Draws a handle on each vertex of the shape for Points mode: vertex 1 of the contour red,
 * vertex 2 green, so that the start and the direction of the contour can be read, the others
 * neutral. Vertex 1 is drawn last, on top when handles coincide (F03.AC4). A selected vertex
 * gets a ring behind its handle (F03.AC5). Their radii, 8 and 13 screen pixels, are divided by the
 * view's scale, so that they keep their size on screen at any zoom (`screenToUser`, ADR-0098,
 * decision 5).
 *
 * @kind procedure
 * @param document - page document
 * @param vertices - vertices of the contour, in drawing order (Q11)
 * @param selected - numbers of the selected vertices, from 0
 * @returns the rings, then one `<path>` per vertex, vertex 1 last
 */
export function createHandleElements(
  document: Document,
  vertices: readonly Point[],
  selected: readonly number[],
): readonly SVGPathElement[] {
  const { scale } = readView(document);
  const halos = vertices
    .filter((vertex, index) => selected.includes(index))
    .map((vertex) => {
      const radius = screenToUser(HALO_RADIUS, scale);
      const halo = createPathElement(document, createCirclePathData(vertex, radius));

      halo.classList.add("halo");

      return halo;
    });
  const handles = vertices.map((vertex, index) => {
    const radius = screenToUser(HANDLE_RADIUS, scale);
    const handle = createPathElement(document, createCirclePathData(vertex, radius));

    handle.classList.add("handle");
    handle.dataset["vertex"] = String(index + 1);

    return handle;
  });

  handles[0]?.classList.add("first");
  handles[1]?.classList.add("second");

  return [...halos, ...handles.toReversed()];
}

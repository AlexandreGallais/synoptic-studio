// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { chooseOption, clickElement, openPage, readPressedState, typeValue } from "./user-actions";

/**
 * Handles drawn on the canvas, in drawing order (the last one on top).
 *
 * @returns the handle paths
 */
function handles(): readonly SVGPathElement[] {
  return [...document.querySelectorAll<SVGPathElement>("#canvas path.handle")];
}

describe("points mode", () => {
  beforeEach(() => {
    // Step 10 of the guided test: a hexagon in 100 × 100 (F02).
    openPage("?step=10");
  });

  it("[F03.AC4] opens on Shape mode, without handles", () => {
    expect(readPressedState("mode-shape")).toBe("true");
    expect(readPressedState("mode-points")).toBe("false");
    expect(handles()).toHaveLength(0);
  });

  it("[F03.AC4] shows one handle per vertex in Points mode, vertex 1 red on top, vertex 2 green", () => {
    clickElement("mode-points");

    expect(readPressedState("mode-points")).toBe("true");
    expect(readPressedState("mode-shape")).toBe("false");

    expect(handles().map((handle) => handle.dataset["vertex"])).toEqual([
      "6",
      "5",
      "4",
      "3",
      "2",
      "1",
    ]);

    expect(handles().at(-1)?.classList.contains("first")).toBe(true);
    expect(handles().at(-2)?.classList.contains("second")).toBe(true);
    // Only one red and one green handle; vertices 3 to 6 neutral.
    expect(document.querySelectorAll("#canvas path.handle.first")).toHaveLength(1);
    expect(document.querySelectorAll("#canvas path.handle.second")).toHaveLength(1);

    // Vertex 1 of the hexagon in 100 × 100, the right end of its base, is (75, 93.30127): a circle
    // of radius 8 around it.
    expect(handles().at(-1)?.getAttribute("d")).toBe(
      "M67 93.30127 A8 8 0 1 1 83 93.30127 A8 8 0 1 1 67 93.30127 Z",
    );
  });

  it("[F03.AC4] hides the handles back in Shape mode", () => {
    clickElement("mode-points");
    clickElement("mode-shape");

    expect(handles()).toHaveLength(0);
  });

  it("[F03.AC4] draws the red handle on top when handles coincide (0 × 50 rectangle)", () => {
    chooseOption("shape", "rectangle");

    typeValue("width", "0");
    typeValue("height", "50");
    clickElement("mode-points");

    // Vertices 1 (0, 0) and 2 (0, 0) coincide: the red one is the last drawn.
    expect(handles().at(-1)?.classList.contains("first")).toBe(true);
    expect(handles()).toHaveLength(4);
  });
});

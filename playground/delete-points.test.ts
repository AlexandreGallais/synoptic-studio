// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { clickElement, clickPoint, openOnStep, readPointRows } from "./user-actions";

/**
 * What the Delete button shows: whether it is greyed, and the line under it.
 *
 * @returns greyed or not, and the line, empty when hidden
 */
function deleteState(): { readonly isGreyed: boolean; readonly line: string } {
  const button = document.querySelector<HTMLButtonElement>("#delete-points");
  const message = document.querySelector<HTMLElement>("#delete-message");
  const isShown = message?.hidden === false;

  return { isGreyed: button?.disabled === true, line: isShown ? message.textContent : "" };
}

/**
 * Presses the Delete key on an element of the page.
 *
 * @param target - element that has the focus
 */
function pressDelete(target: Element | null): void {
  target?.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Delete" }));
}

describe("deleting points", () => {
  it("[F08.AC4] deletes the point selected: its neighbours joined, the selection cleared", () => {
    // Step 1: a 120 × 80 rectangle, radius 10; point 2 (120, 0) deleted.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(2, false);
    clickElement("delete-points");

    expect(readPointRows()).toEqual([
      "1 · x 0 · y 0 · radius 10",
      "2 · x 120 · y 80 · radius 10",
      "3 · x 0 · y 80 · radius 10",
    ]);

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("free");
    expect(document.querySelector<HTMLInputElement>("#point-x")?.value).toBe("");
  });

  it("[F08.AC4] deletes with the Delete key, but not while a field is typed in", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(2, false);
    pressDelete(document.querySelector("#point-x"));

    expect(readPointRows()).toHaveLength(4);

    pressDelete(document.body);

    expect(readPointRows()).toHaveLength(3);
  });

  it("[F08.AC4] greys Delete when no point is selected", () => {
    openOnStep(1);
    clickElement("mode-points");

    expect(deleteState()).toEqual({ isGreyed: true, line: "" });
  });

  it("[F08.AC4] refuses to leave fewer than 3 points, with a line saying why", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);

    expect(deleteState()).toEqual({
      isGreyed: true,
      line: "A shape keeps at least 3 points: select fewer to delete.",
    });
  });

  it("[F08.AC4] refuses to leave points with no height, with a line saying why", () => {
    // A point added in the middle of the top edge: (0, 0), (60, 0), (120, 0), (120, 80), (0, 80);
    // without points 4 and 5, the three left lie at y 0.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);
    clickElement("add-midpoint");
    clickPoint(4, false);
    clickPoint(5, true);

    expect(deleteState()).toEqual({
      isGreyed: true,
      line: "The points left would have no width or no height: select others to delete.",
    });
  });

  it("[F08.AC4] deletes the first and the last points: their neighbours wrap round", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);
    clickElement("add-midpoint");
    clickPoint(1, false);
    clickPoint(5, true);
    clickElement("delete-points");

    expect(readPointRows()).toEqual([
      "1 · x 60 · y 0 · radius 0",
      "2 · x 120 · y 0 · radius 10",
      "3 · x 120 · y 80 · radius 10",
    ]);
  });

  it("[F08.AC4] greys Delete on a polygon and says to convert it first", () => {
    // Step 10: a hexagon.
    openOnStep(10);
    clickElement("mode-points");
    clickPoint(1, false);
    pressDelete(document.body);

    expect(readPointRows()).toHaveLength(6);
    expect(deleteState().isGreyed).toBe(true);
    expect(document.querySelector<HTMLElement>("#convert-hint")?.hidden).toBe(false);
  });

  it("[F08.AC4] deletes nothing while the shape is refused", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(2, false);

    const width = document.querySelector<HTMLInputElement>("#width");

    if (width !== null) {
      width.value = "";
      width.dispatchEvent(new Event("input"));
      pressDelete(document.body);
      width.value = "120";
      width.dispatchEvent(new Event("input"));
    }

    expect(readPointRows()).toHaveLength(4);
  });

  it("[F08.AC4] leaves the rectangle as it is when Delete is pressed with no point selected", () => {
    openOnStep(1);
    clickElement("mode-points");
    pressDelete(document.body);

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
  });

  it("[F08.AC4] deletes with Backspace too, the delete key of a Mac", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(2, false);
    document.body.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Backspace" }));

    expect(readPointRows()).toHaveLength(3);
  });
});

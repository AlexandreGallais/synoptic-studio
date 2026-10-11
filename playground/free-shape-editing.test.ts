// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

/**
 * Opens the playground on a step of the guided test, in Points mode.
 *
 * @param step - number of the step, from 1
 */
function openOnStep(step: number): void {
  globalThis.history.replaceState(undefined, "", `/?step=${String(step)}`);

  const parser = new DOMParser();
  const parsed = parser.parseFromString(page, "text/html");

  parsed.querySelector("script")?.remove();
  document.body.replaceChildren(...parsed.body.childNodes);
  mountPlayground(document);
  document.querySelector<HTMLButtonElement>("#mode-points")?.click();
}

/**
 * Types a value into an input and fires its `input` then `change` events, as a user would who
 * types it and presses Enter.
 *
 * @param id - input id
 * @param value - typed text
 */
function type(id: string, value: string): void {
  const input = document.querySelector<HTMLInputElement>(`#${id}`);

  if (input === null) {
    return;
  }

  input.value = value;
  input.dispatchEvent(new Event("input"));
  input.dispatchEvent(new Event("change"));
}

/**
 * Clicks the row of a point in the list.
 *
 * @param vertex - number of the point, from 1
 */
function clickPoint(vertex: number): void {
  document
    .querySelector(`#points [data-vertex="${CSS.escape(String(vertex))}"]`)
    ?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
}

/**
 * An input of the page.
 *
 * @param id - input id
 * @returns the input, or a detached one when the page has none
 */
function field(id: string): HTMLInputElement {
  return document.querySelector<HTMLInputElement>(`#${id}`) ?? document.createElement("input");
}

/**
 * Text shown in an element of the page.
 *
 * @param selector - CSS selector of the element
 * @returns its text content, empty when absent
 */
function text(selector: string): string {
  return document.querySelector(selector)?.textContent ?? "";
}

/**
 * A number of the model shown in the pipeline panel, written in full by `JSON.stringify`.
 *
 * @param key - name of a top-level number of the model
 * @returns its value, `NaN` when absent
 */
function modelNumber(key: string): number {
  const pattern = new RegExp(String.raw`"${key}": (-?[\d.e+-]+)`, "u");
  const match = pattern.exec(text("#model"));

  return Number(match?.[1]);
}

/**
 * Adds a point of the list to the selection, as Ctrl + click does.
 *
 * @param vertex - number of the point, from 1
 */
function addPoint(vertex: number): void {
  document
    .querySelector(`#points [data-vertex="${CSS.escape(String(vertex))}"]`)
    ?.dispatchEvent(new MouseEvent("click", { bubbles: true, ctrlKey: true }));
}

/**
 * The texts of the rows of the list of points.
 *
 * @returns one text per point, in contour order
 */
function rows(): readonly string[] {
  return [...document.querySelectorAll("#points [data-vertex]")].map((row) => row.textContent);
}

/**
 * Whether the button adding a point is greyed.
 *
 * @returns `true` when it cannot be clicked
 */
function isAddGreyed(): boolean {
  return document.querySelector<HTMLButtonElement>("#add-midpoint")?.disabled ?? false;
}

/**
 * The fieldset holding the radius field.
 *
 * @returns its id, `undefined` when the field sits with the shape's fields
 */
function place(): string | undefined {
  return document.querySelector("#radius")?.closest("fieldset")?.id;
}

describe("editing the points of a free shape", () => {
  it("[F08.AC1] shows the x and y of a polygon's point greyed, and the conversion button", () => {
    // Step 10: a hexagon in 100 × 100.
    openOnStep(10);
    clickPoint(1);

    expect(field("point-x").value).toBe("75");
    expect(field("point-y").value).toBe("93.30127");
    expect(field("point-x").disabled).toBe(true);
    expect(document.querySelector<HTMLButtonElement>("#convert")?.hidden).toBe(false);
  });

  it("[F08.AC1] converts the polygon into a free shape that does not move", () => {
    openOnStep(10);

    const before = text("#path-data");

    document.querySelector<HTMLButtonElement>("#convert")?.click();

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("free");
    expect(text("#path-data")).toBe(before);
    expect(field("width").value).toBe("100");
    expect(field("height").value).toBe("86.60254");

    // The model keeps the exact height, 50√3, not the 5 decimals shown in the input (Q27).
    expect(modelNumber("height")).toBeCloseTo(50 * Math.sqrt(3), 12);
  });

  it("[F08.AC1] hides the anchor, the corners and the conversion of a free shape", () => {
    openOnStep(10);
    document.querySelector<HTMLButtonElement>("#convert")?.click();

    const hidden = ["#anchor-field", "#corners-field", "#convert"].map(
      (selector) => document.querySelector<HTMLElement>(selector)?.hidden,
    );

    expect(hidden).toEqual([true, true, true]);
  });

  it("[F08.AC2] moves a point of the converted polygon to the x typed, its y kept", () => {
    openOnStep(10);
    document.querySelector<HTMLButtonElement>("#convert")?.click();
    clickPoint(1);
    type("point-x", "80");

    expect(text('#points [data-vertex="1"]')).toBe("1 · x 80 · y 93.30127 · radius 0");
    expect(text('#points [data-vertex="3"]')).toBe("3 · x 0 · y 50 · radius 0");
  });

  it("[F08.AC2] makes the rectangle a free shape when one of its points is typed", () => {
    // Step 1: a 120 × 80 rectangle with radius 10.
    openOnStep(1);
    clickPoint(3);
    type("point-x", "150");

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("free");
    expect(text('#points [data-vertex="3"]')).toBe("3 · x 150 · y 80 · radius 10");
    // The box around the points: 150 wide now.
    expect(field("width").value).toBe("150");
  });

  it("[F08.AC2] ignores a coordinate that is not an integer", () => {
    openOnStep(1);
    clickPoint(3);
    type("point-x", "150.5");

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
    expect(text('#points [data-vertex="3"]')).toBe("3 · x 120 · y 80 · radius 10");
  });

  it("[F08.AC1] stretches the converted polygon by an integer width, and refuses a fractional one", () => {
    openOnStep(10);
    document.querySelector<HTMLButtonElement>("#convert")?.click();
    type("width", "50");

    expect(text('#points [data-vertex="1"]')).toBe("1 · x 37.5 · y 93.30127 · radius 0");

    type("width", "12.5");

    expect(text("#status")).toBe(
      "Width, height and radius typed must be integers ≥ 0, and the points must span some width and height.",
    );
  });

  it("[F08.AC1] greys the conversion of a polygon of width 0", () => {
    openOnStep(10);
    type("width", "0");

    expect(document.querySelector<HTMLButtonElement>("#convert")?.disabled).toBe(true);
  });

  it("[F08.AC2] hides the point fields and moves nothing while the shape is refused", () => {
    // Step 1, point 3 typed: a free shape; its width then cleared, refused.
    openOnStep(1);
    clickPoint(3);
    type("point-x", "150");
    type("width", "");

    expect(document.querySelector<HTMLElement>("#point-fields")?.hidden).toBe(true);

    type("point-x", "40");
    type("width", "150");

    // The point stayed at 150: the shape kept is intact, drawn again.
    expect(text('#points [data-vertex="3"]')).toBe("3 · x 150 · y 80 · radius 10");
  });

  it("[F08.AC1] converts no refused polygon", () => {
    // Step 10 with 2 corners: no polygon.
    openOnStep(10);
    type("corners", "2");
    document.querySelector<HTMLButtonElement>("#convert")?.click();

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("polygon");
  });

  it("[F08.AC2] moves nothing when no point is selected", () => {
    openOnStep(1);
    clickPoint(3);
    document.querySelector<HTMLSelectElement>("#shape")?.dispatchEvent(new Event("change"));
    type("point-x", "5");

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
  });

  it("[F08.AC2] drops the free shape kept when a guided step draws its own", () => {
    openOnStep(1);
    clickPoint(3);
    type("point-x", "150");
    document.querySelector<HTMLButtonElement>("#guide-next")?.click();

    // A step draws its own shape: the free shape goes, and so does its option.
    expect(
      document.querySelector<HTMLSelectElement>("#shape")?.dataset["freeShape"],
    ).toBeUndefined();

    expect(document.querySelector<HTMLOptionElement>('#shape option[value="free"]')?.hidden).toBe(
      true,
    );
  });

  it("[F08.AC3] adds a sharp point in the middle of the edge of two neighbours, selected alone", () => {
    // Step 1: a 120 × 80 rectangle, radius 10; points 1 (0, 0) and 2 (120, 0): middle (60, 0).
    openOnStep(1);
    clickPoint(1);
    addPoint(2);
    document.querySelector<HTMLButtonElement>("#add-midpoint")?.click();

    expect(rows()).toEqual([
      "1 · x 0 · y 0 · radius 10",
      "2 · x 60 · y 0 · radius 0",
      "3 · x 120 · y 0 · radius 10",
      "4 · x 120 · y 80 · radius 10",
      "5 · x 0 · y 80 · radius 10",
    ]);

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("free");
    expect(field("point-x").value).toBe("60");
  });

  it("[F08.AC3] adds the point of the closing edge, between the last point and the first, at the end", () => {
    // Points 4 (0, 80) and 1 (0, 0): middle (0, 40), numbered 5.
    openOnStep(1);
    clickPoint(4);
    addPoint(1);
    document.querySelector<HTMLButtonElement>("#add-midpoint")?.click();

    expect(rows().at(-1)).toBe("5 · x 0 · y 40 · radius 0");
    expect(rows()).toHaveLength(5);
  });

  it("[F08.AC3] halves the room of the corners at the ends of the edge", () => {
    // A 100 square, radius 1000 drawn 100; a point in the middle of the top edge leaves 50.
    openOnStep(1);
    type("width", "100");
    type("height", "100");
    clickPoint(1);
    addPoint(2);
    // Points 1 and 2 selected: the radius goes to both.
    type("radius", "1000");
    document.querySelector<HTMLButtonElement>("#add-midpoint")?.click();

    expect(rows()[0]).toBe("1 · x 0 · y 0 · radius 1000 (effective 50)");
  });

  it("[F08.AC3] greys the button unless two neighbouring points are selected", () => {
    openOnStep(1);
    clickPoint(1);

    const isOneGreyed = isAddGreyed();

    addPoint(3);

    expect([isOneGreyed, isAddGreyed()]).toEqual([true, true]);
  });

  it("[F08.AC3] greys the button on a polygon and says to convert it first", () => {
    openOnStep(10);
    clickPoint(1);
    addPoint(2);

    expect(isAddGreyed()).toBe(true);
    expect(document.querySelector<HTMLElement>("#convert-hint")?.hidden).toBe(false);
  });

  it("[F08.AC3] adds no point while the shape is refused", () => {
    openOnStep(1);
    clickPoint(1);
    addPoint(2);
    type("width", "");
    document.querySelector<HTMLButtonElement>("#add-midpoint")?.click();
    type("width", "120");

    expect(rows()).toHaveLength(4);
  });

  it("[F08.AC2] keeps the free shape when a point typed would leave it no width", () => {
    // A triangle (0, 0), (120, 80), (0, 80) once point 2 is deleted; point 2 typed at x 0 would put
    // every point at x 0: refused, the triangle stays.
    openOnStep(1);
    clickPoint(2);
    document.querySelector<HTMLButtonElement>("#delete-points")?.click();
    clickPoint(2);
    type("point-x", "0");

    expect(rows()).toEqual([
      "1 · x 0 · y 0 · radius 10",
      "2 · x 120 · y 80 · radius 10",
      "3 · x 0 · y 80 · radius 10",
    ]);

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("free");
  });

  it("[F08.AC3] greys adding a point on a shape of width 0, its radii kept", () => {
    openOnStep(1);
    type("width", "0");
    clickPoint(1);
    addPoint(2);
    document.querySelector<HTMLButtonElement>("#add-midpoint")?.click();

    expect(isAddGreyed()).toBe(true);
    expect(rows()[1]).toBe("2 · x 0 · y 0 · radius 10");
    // Points 1 and 2 still selected: no single point to show.
    expect(field("point-x").value).toBe("");
  });

  it("[F08.AC5] puts the radius with the point fields in Points mode, with the shape's otherwise", () => {
    openOnStep(1);

    const inPoints = place();

    type("width", "");

    const whenRefused = place();

    type("width", "120");
    document.querySelector<HTMLButtonElement>("#mode-shape")?.click();

    expect([inPoints, whenRefused, place()]).toEqual(["point-fields", undefined, undefined]);
  });
});

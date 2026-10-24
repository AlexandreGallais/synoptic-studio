// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  commitField,
  openOnStep,
  readDisabledState,
  readPointRows,
  readText,
  readValue,
  selectInput,
  typeAndEnter,
  typeInField,
} from "../user-actions";

/**
 * A number of the model shown in the pipeline panel, written in full by `JSON.stringify`.
 *
 * @param key - name of a top-level number of the model
 * @returns its value, `NaN` when absent
 */
function modelNumber(key: string): number {
  const pattern = new RegExp(String.raw`"${key}": (-?[\d.e+-]+)`, "u");
  const match = pattern.exec(readText("#model"));

  return Number(match?.[1]);
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
    clickElement("mode-points");
    clickPoint(1, false);

    expect(selectInput("point-x").value).toBe("75");
    expect(selectInput("point-y").value).toBe("93.30127");
    expect(selectInput("point-x").disabled).toBe(true);
    expect(document.querySelector<HTMLButtonElement>("#convert")?.hidden).toBe(false);
  });

  it("[F08.AC1] converts the polygon into a free shape that does not move", () => {
    openOnStep(10);
    clickElement("mode-points");

    const before = readText("#path-data");

    clickElement("convert");

    expect(readValue("shape")).toBe("free");
    expect(readText("#path-data")).toBe(before);
    expect(selectInput("width").value).toBe("100");
    expect(selectInput("height").value).toBe("86.60254");

    // The model keeps the exact height, 50√3, not the 5 decimals shown in the input (Q27).
    expect(modelNumber("height")).toBeCloseTo(50 * Math.sqrt(3), 12);
  });

  it("[F08.AC1] hides the anchor, the corners and the conversion of a free shape", () => {
    openOnStep(10);
    clickElement("mode-points");
    clickElement("convert");

    const hidden = ["#anchor-field", "#corners-field", "#convert"].map(
      (selector) => document.querySelector<HTMLElement>(selector)?.hidden,
    );

    expect(hidden).toEqual([true, true, true]);
  });

  it("[F08.AC2] moves a point of the converted polygon to the x typed, its y kept", () => {
    openOnStep(10);
    clickElement("mode-points");
    clickElement("convert");
    clickPoint(1, false);
    typeAndEnter("point-x", "80");

    expect(readText('#points [data-vertex="1"]')).toBe("1 · x 80 · y 93.30127 · radius 0");
    expect(readText('#points [data-vertex="3"]')).toBe("3 · x 0 · y 50 · radius 0");
  });

  it("[F08.AC2] makes the rectangle a free shape when one of its points is typed", () => {
    // Step 1: a 120 × 80 rectangle with radius 10.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(3, false);
    typeAndEnter("point-x", "150");

    expect(readValue("shape")).toBe("free");
    expect(readText('#points [data-vertex="3"]')).toBe("3 · x 150 · y 80 · radius 10");
    // The box around the points: 150 wide now.
    expect(selectInput("width").value).toBe("150");
  });

  it("[F08.AC2] ignores a coordinate that is no number, its x shown again", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(3, false);
    typeInField("point-x", "abc");

    expect([readValue("shape"), readValue("point-x")]).toEqual(["rectangle", "120"]);
    expect(readText('#points [data-vertex="3"]')).toBe("3 · x 120 · y 80 · radius 10");
  });

  it("[F44.AC4] rounds a coordinate typed with a fraction: 150.5 gives 151, a tie up", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(3, false);
    typeAndEnter("point-x", "150.5");

    expect(readValue("shape")).toBe("free");
    expect(readText('#points [data-vertex="3"]')).toBe("3 · x 151 · y 80 · radius 10");
  });

  it("[F08.AC1] stretches the converted polygon by an integer width, a fraction typed rounded", () => {
    openOnStep(10);
    clickElement("mode-points");
    clickElement("convert");
    typeAndEnter("width", "50");

    expect(readText('#points [data-vertex="1"]')).toBe("1 · x 37.5 · y 93.30127 · radius 0");

    // A fraction typed is rounded, a tie up (AUD-046): 12.5 gives 13.
    typeAndEnter("width", "12.5");

    expect(readValue("width")).toBe("13");

    // A radius emptied by a script, which the field never read: refused.
    typeAndEnter("radius", "");

    expect(readText("#status")).toBe(
      "Width, height and radius typed must be integers ≥ 0, the angle an integer, and the points must span some width and height.",
    );
  });

  it("[F08.AC1] greys the conversion of a polygon of width 0", () => {
    openOnStep(10);
    clickElement("mode-points");
    typeAndEnter("width", "0");

    expect(document.querySelector<HTMLButtonElement>("#convert")?.disabled).toBe(true);
  });

  it("[F08.AC2] hides the point fields and moves nothing while the shape is refused", () => {
    // Step 1, point 3 typed: a free shape; its angle then cleared, refused.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(3, false);
    typeAndEnter("point-x", "150");
    typeAndEnter("angle", "");

    expect(document.querySelector<HTMLElement>("#point-fields")?.hidden).toBe(true);

    typeAndEnter("point-x", "40");
    typeAndEnter("angle", "0");

    // The point stayed at 150: the shape kept is intact, drawn again.
    expect(readText('#points [data-vertex="3"]')).toBe("3 · x 150 · y 80 · radius 10");
  });

  it("[F08.AC1] converts no refused polygon", () => {
    // Step 10, its corners emptied by a script: no polygon.
    openOnStep(10);
    clickElement("mode-points");
    typeAndEnter("corners", "");
    clickElement("convert");

    expect(readValue("shape")).toBe("polygon");
  });

  it("[F08.AC2] moves nothing when no point is selected", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(3, false);
    commitField("shape");
    typeAndEnter("point-x", "5");

    expect(readValue("shape")).toBe("rectangle");
  });

  it("[F08.AC2] drops the free shape kept when a guided step draws its own", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(3, false);
    typeAndEnter("point-x", "150");
    clickElement("guide-next");

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
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);
    clickElement("add-midpoint");

    expect(readPointRows()).toEqual([
      "1 · x 0 · y 0 · radius 10",
      "2 · x 60 · y 0 · radius 0",
      "3 · x 120 · y 0 · radius 10",
      "4 · x 120 · y 80 · radius 10",
      "5 · x 0 · y 80 · radius 10",
    ]);

    expect(readValue("shape")).toBe("free");
    expect(selectInput("point-x").value).toBe("60");
  });

  it("[F08.AC3] adds the point of the closing edge, between the last point and the first, at the end", () => {
    // Points 4 (0, 80) and 1 (0, 0): middle (0, 40), numbered 5.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(4, false);
    clickPoint(1, true);
    clickElement("add-midpoint");

    expect(readPointRows().at(-1)).toBe("5 · x 0 · y 40 · radius 0");
    expect(readPointRows()).toHaveLength(5);
  });

  it("[F08.AC3] halves the room of the corners at the ends of the edge", () => {
    // A 100 square, radius 1000 drawn 100; a point in the middle of the top edge leaves 50.
    openOnStep(1);
    clickElement("mode-points");
    typeAndEnter("width", "100");
    typeAndEnter("height", "100");
    clickPoint(1, false);
    clickPoint(2, true);
    // Points 1 and 2 selected: the radius goes to both.
    typeAndEnter("radius", "1000");
    clickElement("add-midpoint");

    expect(readPointRows()[0]).toBe("1 · x 0 · y 0 · radius 1000 (effective 50)");
  });

  it("[F08.AC3] greys the button unless two neighbouring points are selected", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(1, false);

    const isOneGreyed = readDisabledState("add-midpoint") === "true";

    clickPoint(3, true);

    expect([isOneGreyed, readDisabledState("add-midpoint") === "true"]).toEqual([true, true]);
  });

  it("[F08.AC3] greys the button on a polygon and says to convert it first", () => {
    openOnStep(10);
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);

    expect(readDisabledState("add-midpoint") === "true").toBe(true);
    expect(document.querySelector<HTMLElement>("#convert-hint")?.hidden).toBe(false);
  });

  it("[F08.AC3] adds no point while the shape is refused", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);
    typeAndEnter("angle", "");
    clickElement("add-midpoint");
    typeAndEnter("angle", "0");

    expect(readPointRows()).toHaveLength(4);
  });

  it("[F08.AC2] keeps the free shape when a point typed would leave it no width", () => {
    // A triangle (0, 0), (120, 80), (0, 80) once point 2 is deleted; point 2 typed at x 0 would put
    // every point at x 0: refused, the triangle stays.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(2, false);
    clickElement("delete-points");
    clickPoint(2, false);
    typeAndEnter("point-x", "0");

    expect(readPointRows()).toEqual([
      "1 · x 0 · y 0 · radius 10",
      "2 · x 120 · y 80 · radius 10",
      "3 · x 0 · y 80 · radius 10",
    ]);

    expect(readValue("shape")).toBe("free");
  });

  it("[F08.AC3] greys adding a point on a shape of width 0, its radii kept", () => {
    openOnStep(1);
    clickElement("mode-points");
    typeAndEnter("width", "0");
    clickPoint(1, false);
    clickPoint(2, true);
    clickElement("add-midpoint");

    expect(readDisabledState("add-midpoint") === "true").toBe(true);
    expect(readPointRows()[1]).toBe("2 · x 0 · y 0 · radius 10");
    // Points 1 and 2 still selected: no single point to show.
    expect(selectInput("point-x").value).toBe("");
  });

  it("[F08.AC5] shows the shape's radius in both modes, and a radius of the points in Points mode", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(1, false);
    typeAndEnter("point-radius", "30");

    // The shape's field still speaks for every corner: 30 and 10, Mixed.
    expect([place(), selectInput("radius").placeholder]).toEqual([undefined, "Mixed"]);

    typeAndEnter("radius", "5");

    expect(readPointRows().map((row) => row.endsWith("radius 5"))).toEqual([
      true,
      true,
      true,
      true,
    ]);
  });

  it("[F08.AC2] keeps the focus in a radius field while it is typed in", () => {
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(1, false);

    for (const id of ["point-radius", "radius"]) {
      selectInput(id).focus();
      typeAndEnter(id, "1");
      typeAndEnter(id, "12");

      expect(document.activeElement === selectInput(id)).toBe(true);
    }
  });

  it("[F08.AC2] brings the free shape back, its own size, when it is chosen again", () => {
    // Step 1: point 3 typed at x 150, a free shape 150 wide; then the polygon, 100 wide, then back.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(3, false);
    typeAndEnter("point-x", "150");

    for (const value of ["polygon", "free"]) {
      chooseOption("shape", value);

      // The polygon gets a width of its own, which the free shape must not take.
      typeAndEnter("width", value === "polygon" ? "100" : selectInput("width").value);
    }

    expect(selectInput("width").value).toBe("150");
    expect(readPointRows()[2]).toBe("3 · x 150 · y 80 · radius 10");
  });
});

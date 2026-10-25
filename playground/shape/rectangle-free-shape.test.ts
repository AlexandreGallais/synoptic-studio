// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { curveCornerOf } from "../../src";
import {
  clickElement,
  clickPoint,
  openPage,
  readShapeModel,
  readText,
  readValue,
  typeAndEnter,
} from "../user-actions";

describe("rectangle as a free shape", () => {
  beforeEach(() => {
    // The page as it opens: a 120 × 80 rectangle, radius 10 (F01).
    openPage("");
  });

  it("[F09.AC4] shows the rectangle as a free shape: the corners of the unit square with their radii, its size", () => {
    const model: unknown = JSON.parse(readText("#model"));

    expect(model).toEqual({
      angle: 0,
      corners: [
        { point: { x: 0, y: 0 }, radius: 10 },
        { point: { x: 1, y: 0 }, radius: 10 },
        { point: { x: 1, y: 1 }, radius: 10 },
        { point: { x: 0, y: 1 }, radius: 10 },
      ].map((corner) => curveCornerOf(corner)),
      height: 80,
      width: 120,
      x: 0,
      y: 0,
    });
  });

  it("[F09.AC3] keeps the stored points at width 0, and comes back the same", () => {
    const before = readText("#path-data");
    const model = readText("#model");

    typeAndEnter("width", "0");

    // Nothing drawn across, the stored unit square untouched.
    expect(readText("#path-data")).toBe("M0 0 L0 80 Z");

    expect(JSON.parse(readText("#model"))).toMatchObject({
      corners: [
        { point: { x: 0, y: 0 } },
        { point: { x: 1, y: 0 } },
        { point: { x: 1, y: 1 } },
        { point: { x: 0, y: 1 } },
      ],
      width: 0,
    });

    typeAndEnter("width", "120");

    expect(readText("#model")).toBe(model);
    expect(readText("#path-data")).toBe(before);

    expect(before).toBe(
      "M10 0 L110 0 A10 10 0 0 1 120 10 L120 70 A10 10 0 0 1 110 80 L10 80 A10 10 0 0 1 0 70 L0 10 A10 10 0 0 1 10 0 Z",
    );
  });

  it("[F09.AC4] lets Points mode round one corner of the rectangle, as in F03", () => {
    clickElement("mode-points");

    clickPoint(3, false);

    typeAndEnter("point-radius", "30");

    expect(document.querySelector<HTMLInputElement>("#radius")?.dataset["radii"]).toBe(
      "10 10 30 10",
    );

    expect(readText("#path-data")).toContain("A30 30");
  });

  it("[F09.AC4] lists the drawn points of the rectangle, not the stored ones", () => {
    clickElement("mode-points");

    expect(readText('#points [data-vertex="1"]')).toBe("1 · x 0 · y 0 · radius 10");

    expect(readText('#points [data-vertex="3"]')).toBe("3 · x 120 · y 80 · radius 10");
  });

  it("[F09.AC4] keeps the rectangle's width an integer, 2.5 typed rounded to 3 (Q15)", () => {
    // A fraction typed is rounded once, a tie up: the rectangle stays on integers.
    typeAndEnter("width", "2.5");

    expect([readValue("width"), readShapeModel().width, readText("#status")]).toEqual(["3", 3, ""]);
  });

  it("[F09.AC4] keeps the rectangle's radius an integer, 2.5 typed rounded to 3 (Q15)", () => {
    typeAndEnter("radius", "2.5");

    expect([readValue("radius"), readText("#status")]).toEqual(["3", ""]);
    expect(readText("#path-data")).toContain("A3 3");
  });
});

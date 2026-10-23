import { describe, expect, it } from "vitest";

import { keptRectangleAt } from "../../../tests";

import { shapeTextLength } from "./shape-text-length";

describe("shapeTextLength (F40)", () => {
  it.each([
    // keptRectangleAt's TSDoc: 614 characters at x 10, 615 at x 200.
    { length: 614, shape: keptRectangleAt(10), title: "A, at x 10" },
    { length: 615, shape: keptRectangleAt(200), title: "B, at x 200" },
    // ["a"]: the 5 characters `["a"]`.
    { length: 5, shape: ["a"], title: "a list of one letter" },
    // []: the 2 characters `[]`.
    { length: 2, shape: [], title: "an empty list" },
  ])("[F40.AC4] measures $title: $length", ({ length, shape }) => {
    expect(shapeTextLength(shape)).toBe(length);
  });

  it("[F40.AC4] writes a shape's text once, however often it is measured", () => {
    // `toJSON` is called each time `JSON.stringify` writes the shape: `{"x":1}`, 7 characters.
    const written: string[] = [];
    const shape = {
      toJSON: (): object => {
        written.push("x");

        return { x: 1 };
      },
    };

    expect([shapeTextLength(shape), shapeTextLength(shape)]).toEqual([7, 7]);
    expect(written).toHaveLength(1);
  });
});

import { describe, expect, it } from "vitest";

import { keptRectangleAt } from "../../../tests";

import { memorizedShape } from "./memorized-shape";

describe("memorizedShape (ADR-0096, ADR-0097)", () => {
  it("[F41.AC2] keeps A's key and its 614 characters, the same memo at every call", () => {
    // keptRectangleAt's TSDoc: 614 characters at x 10.
    const shape = keptRectangleAt(10);
    const memo = memorizedShape(shape);

    expect(memo.length).toBe(614);
    expect(memorizedShape(shape)).toBe(memo);
  });

  it("[F41.AC2] gives each new shape the next key, whatever its text", () => {
    // Two objects of the text `["a"]`, 5 characters, then `[]`, 2: three keys in a row.
    const memos = [["a"], ["a"], []].map((shape) => memorizedShape(shape));
    const [first] = memos;

    expect(memos.map(({ key, length }) => ({ key: key - (first?.key ?? 0), length }))).toEqual([
      { key: 0, length: 5 },
      { key: 1, length: 5 },
      { key: 2, length: 2 },
    ]);
  });

  it("[F41.AC2] writes a shape's text once, however often it is read", () => {
    // `toJSON` is called each time `JSON.stringify` writes the shape: `{"x":1}`, 7 characters.
    const written: string[] = [];
    const shape = {
      toJSON: (): object => {
        written.push("x");

        return { x: 1 };
      },
    };

    expect([memorizedShape(shape).length, memorizedShape(shape).length]).toEqual([7, 7]);
    expect(written).toHaveLength(1);
  });
});

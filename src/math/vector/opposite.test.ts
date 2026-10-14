import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { add } from "./add";
import { opposite } from "./opposite";

describe("opposite (REF-MATHWORLD-VECTOR-DIFFERENCE)", () => {
  it("reverses both components", () => {
    expect(opposite({ x: 3, y: -4 })).toEqual({ x: -3, y: 4 });
  });

  test.prop({ v: integerPoint(1e4) })("adds to the vector to give zero", ({ v }) => {
    const sum = add(v, opposite(v));

    expect([sum.x + 0, sum.y + 0]).toEqual([0, 0]);
  });
});

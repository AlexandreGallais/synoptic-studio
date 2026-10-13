import { describe, expect, it } from "vitest";

import { add } from "./add";
import { ZERO_VECTOR } from "./zero-vector";

describe("ZERO_VECTOR", () => {
  it("leaves a vector unchanged when added", () => {
    expect(add({ x: 3, y: -4 }, ZERO_VECTOR)).toEqual({ x: 3, y: -4 });
  });
});

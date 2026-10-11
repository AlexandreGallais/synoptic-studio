import { describe, expect, it } from "vitest";

import { boxCenter } from "./box-center";

describe("boxCenter (ADR-0035)", () => {
  it("[F04.AC3] is the position plus half the size: (10, 20) and 100 × 50 give (60, 45)", () => {
    expect(boxCenter({ height: 50, width: 100, x: 10, y: 20 })).toEqual({ x: 60, y: 45 });
  });
});

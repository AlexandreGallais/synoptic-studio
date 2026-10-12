import { describe, expect, it } from "vitest";

import { EPSILON } from "./epsilon";

describe("EPSILON (Q10)", () => {
  it("is 10⁻⁹, the tolerance settled by the Product Owner", () => {
    expect(EPSILON).toBe(1e-9);
  });
});

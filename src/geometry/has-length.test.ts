import { describe, expect, it } from "vitest";

import { EPSILON } from "../math";

import { hasLength } from "./has-length";

describe("hasLength (EPSILON, Q10)", () => {
  it("keeps a piece whose ends are exactly EPSILON apart", () => {
    expect(hasLength({ end: { x: EPSILON, y: 0 }, kind: "segment", start: { x: 0, y: 0 } })).toBe(
      true,
    );
  });

  it("drops a piece whose ends are closer than EPSILON", () => {
    expect(
      hasLength({ end: { x: EPSILON / 2, y: 0 }, kind: "segment", start: { x: 0, y: 0 } }),
    ).toBe(false);
  });

  it("drops the degenerate arc of a corner without fillet", () => {
    const vertex = { x: 50, y: 0 };

    expect(
      hasLength({
        center: vertex,
        end: vertex,
        kind: "arc",
        positive: false,
        radius: 30,
        start: vertex,
      }),
    ).toBe(false);
  });
});

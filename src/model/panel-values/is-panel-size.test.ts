import { describe, expect, it } from "vitest";

import { isPanelSize } from "./is-panel-size";

import type { PanelValues } from "./panel-values";

/** The four fields of several shapes, X, Y, W and H. */
const KEYS: readonly (keyof PanelValues)[] = ["x", "y", "width", "height"];

describe("isPanelSize (F38)", () => {
  it("tells W and H, sizes, from X and Y, positions", () => {
    expect(KEYS.map((key) => isPanelSize(key))).toEqual([false, false, true, true]);
  });
});

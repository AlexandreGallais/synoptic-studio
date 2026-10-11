import { describe, expect, it } from "vitest";

import { radiiWithSelection } from "./radii-with-selection";

describe("radiiWithSelection", () => {
  it("[F03.AC5] gives the radius typed to the selected vertices only", () => {
    expect(radiiWithSelection([0, 0, 0, 0, 0, 0], [0, 3], 10)).toEqual([10, 0, 0, 10, 0, 0]);
  });

  it("[F03.AC5] changes nothing when no vertex is selected", () => {
    expect(radiiWithSelection([5, 6, 7], [], 10)).toEqual([5, 6, 7]);
  });
});

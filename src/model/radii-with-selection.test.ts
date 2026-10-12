import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { radiiWithSelection } from "./radii-with-selection";

describe("radiiWithSelection", () => {
  it("[F03.AC5] gives the radius typed to the selected vertices only", () => {
    expect(radiiWithSelection([0, 0, 0, 0, 0, 0], [0, 3], 10)).toEqual([10, 0, 0, 10, 0, 0]);
  });

  it("[F03.AC5] changes nothing when no vertex is selected", () => {
    expect(radiiWithSelection([5, 6, 7], [], 10)).toEqual([5, 6, 7]);
  });

  test.prop([
    fc.array(fc.nat({ max: 1000 }), { maxLength: 12 }),
    fc.uniqueArray(fc.nat({ max: 11 })),
    fc.nat({ max: 1000 }),
  ])(
    "keeps the number of radii, gives the selected vertices the radius, leaves the others",
    (radii, selected, radius) => {
      const after = radiiWithSelection(radii, selected, radius);

      expect(after).toHaveLength(radii.length);

      expect(
        after.every((value, index) => value === (selected.includes(index) ? radius : radii[index])),
      ).toBe(true);
    },
  );
});

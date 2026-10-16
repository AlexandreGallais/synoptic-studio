import { describe, expect, it } from "vitest";

import { CHECKLIST_LINE_LIMIT } from "./checklist-line-limit";
import { CHECKLISTS } from "./checklists";
import { readFeaturesOverLimit } from "./read-features-over-limit";

import type { ChecklistLine } from "./checklist-line";

/**
 * Lines of a feature, each with its own thing to try.
 *
 * @param feature - the one the lines check, `F05`
 * @param count - how many lines
 * @returns that many lines, each with its own thing to try
 */
function createLines(feature: string, count: number): readonly ChecklistLine[] {
  return Array.from({ length: count }, (unused, index) => ({
    feature,
    see: "seen",
    todo: `${feature} try ${String(index)}`,
  }));
}

describe("[F18.AC12] at most 12 lines per feature in an epic's checklist (EN-056)", () => {
  it("is 12", () => {
    expect(CHECKLIST_LINE_LIMIT).toBe(12);
  });

  it.each([...CHECKLISTS])("holds in %s's checklist", (name, lines) => {
    // Fails naming each feature over the limit, `F05: 14 lines`.
    expect(readFeaturesOverLimit(lines, CHECKLIST_LINE_LIMIT), name).toEqual([]);
  });

  it("names a feature of 13 lines, not one of 12 nor one of 0", () => {
    const lines = [...createLines("F01", 12), ...createLines("F02", 13), ...createLines("F03", 1)];

    expect(readFeaturesOverLimit(lines, 12)).toEqual(["F02: 13 lines"]);
    expect(readFeaturesOverLimit([], 12)).toEqual([]);
  });

  it("counts a feature's lines wherever they sit, and names each feature over in order", () => {
    // F07: 2 + 2 = 4 lines, F06: 3; both over 2, F07 first as it comes first.
    const lines = [
      ...createLines("F07", 2),
      ...createLines("F06", 3),
      ...createLines("F07", 2),
      ...createLines("F08", 2),
    ];

    expect(readFeaturesOverLimit(lines, 2)).toEqual(["F07: 4 lines", "F06: 3 lines"]);
  });
});

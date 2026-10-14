import { describe, expect, it } from "vitest";

import { featureLine, storyRows } from "./session-state";

/** Story table of a feature with one finished story out of three. */
const TABLE = [
  "| ID | Type | Title | Status |",
  "| --- | --- | --- | --- |",
  "| [EN-001](../stories/a.md) | Enabler | First | done |",
  "| [SP-001](../stories/b.md) | Spike | Research | draft |",
  "| [VAL-001](../stories/c.md) | Validation | Validate | draft |",
].join("\n");

describe("session-state hook", () => {
  it("reads the story rows of a feature in delivery order", () => {
    expect(storyRows(TABLE).map((row) => `${row.id} ${row.status}`)).toEqual([
      "EN-001 done",
      "SP-001 draft",
      "VAL-001 draft",
    ]);
  });

  it("names the next unfinished story of a started feature", () => {
    expect(featureLine("E01-F01-x.md", storyRows(TABLE))).toBe(
      "E01-F01-x: 1/3 stories finished; next SP-001 (draft) — Research",
    );
  });

  it("sees a feature started by its first story in progress", () => {
    expect(
      featureLine("E14-F11-x.md", [
        { id: "SP-010", status: "in-progress", title: "Research" },
        { id: "VAL-010", status: "draft", title: "Validate" },
      ]),
    ).toBe("E14-F11-x: 0/2 stories finished; next SP-010 (in-progress) — Research");
  });

  it("is silent for a feature not started or finished", () => {
    expect(featureLine("f.md", [{ id: "US-001", status: "draft", title: "t" }])).toBeUndefined();
    expect(featureLine("f.md", [{ id: "US-001", status: "done", title: "t" }])).toBeUndefined();
  });
});

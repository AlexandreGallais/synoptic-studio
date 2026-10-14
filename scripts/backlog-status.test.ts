import { describe, expect, it } from "vitest";

import { folderOf, withFrontStatus, withRowStatus } from "./backlog-status";

describe("backlog-status", () => {
  it("changes the front matter's status only", () => {
    expect(withFrontStatus("---\nid: US-030\nstatus: draft\n---\nstatus: kept", "done")).toBe(
      "---\nid: US-030\nstatus: done\n---\nstatus: kept",
    );
  });

  it("changes the last cell of the row linking to the item, not another row", () => {
    const table = [
      "| ID | Title | Status |",
      "| [US-030](./a.md) | Convert | in-progress |",
      "| [US-031](./b.md) | Other, see [US-030] | draft |",
    ].join("\n");

    expect(withRowStatus(table, "US-031", "done").split("\n")).toEqual([
      "| ID | Title | Status |",
      "| [US-030](./a.md) | Convert | in-progress |",
      "| [US-031](./b.md) | Other, see [US-030] | done |",
    ]);
  });

  it("finds the folder from the form of the id", () => {
    expect([folderOf("E14"), folderOf("F10"), folderOf("RET-002")]).toEqual([
      "epics",
      "features",
      "stories",
    ]);
  });
});

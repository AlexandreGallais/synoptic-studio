import { describe, expect, it } from "vitest";

import { recordCopies, recordsFolder } from "./records-save";

describe("records-save", () => {
  it("puts the records beside the repository unless RECORDS_PATH is set", () => {
    expect(recordsFolder(undefined, "/work/SVG editor")).toBe("/work/synoptic-studio-records");
    expect(recordsFolder("", "/work/SVG editor")).toBe("/work/synoptic-studio-records");
    expect(recordsFolder("/backup/records", "/work/SVG editor")).toBe("/backup/records");
  });

  it("saves the metrics, the transcripts of this project and the library", () => {
    expect(recordCopies("/work/SVG editor", "/home/me", "/data/library.sqlite")).toEqual([
      {
        from: "/work/SVG editor/.git/metrics/tool-calls.jsonl",
        kind: "file",
        to: "metrics/tool-calls.jsonl",
      },
      {
        from: "/work/SVG editor/.git/metrics/turns.jsonl",
        kind: "file",
        to: "metrics/turns.jsonl",
      },
      {
        // Claude Code names a project's folder after its path, every other character a dash.
        from: "/home/me/.claude/projects/-work-SVG-editor",
        kind: "folder",
        to: "transcripts",
      },
      { from: "/data/library.sqlite", kind: "database", to: "library/library.sqlite" },
    ]);
  });
});

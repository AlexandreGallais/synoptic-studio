import { describe, expect, it } from "vitest";

import { applyEdits, parseEdits } from "./replace-text";

describe("replace-text", () => {
  it("reads blocks of file, old text and new text, multi-line and empty included", () => {
    const spec = "@@ a.ts\n<<<\none\ntwo\n===\nthree\n>>>\n@@ b.md\n<<<\nfour\n===\n>>>\n";

    expect(parseEdits(spec)).toEqual([
      { file: "a.ts", next: "three", previous: "one\ntwo" },
      { file: "b.md", next: "", previous: "four" },
    ]);
  });

  it("replaces each old text found once, in order, `$` kept as written", () => {
    const edits = [
      { file: "f", next: "b$&", previous: "a" },
      { file: "f", next: "c", previous: "b$&" },
    ];

    expect(applyEdits("xay", edits)).toEqual({ problem: "", text: "xcy" });
  });

  it("refuses an old text missing, repeated or empty, the text left as it was", () => {
    expect(applyEdits("aa", [{ file: "f", next: "b", previous: "a" }])).toEqual({
      problem: "edit 1: found 2 times",
      text: "aa",
    });

    expect(applyEdits("x", [{ file: "f", next: "b", previous: "a" }]).problem).toBe(
      "edit 1: found 0 times",
    );

    expect(applyEdits("x", [{ file: "f", next: "b", previous: "" }]).problem).toBe(
      "edit 1: empty old text",
    );
  });
});

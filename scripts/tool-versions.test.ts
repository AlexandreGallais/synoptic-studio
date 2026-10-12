import { describe, expect, it } from "vitest";

import { actionUses, hasMajor, latestLtsMajor } from "./tool-versions";

describe("actionUses", () => {
  it("reads each action and its major version, a sub-path included", () => {
    const workflow = [
      "      - uses: actions/checkout@v7",
      "      - uses: github/codeql-action/init@v4",
      "        uses: googleapis/release-please-action@v5",
      "      - run: npm ci",
    ].join("\n");

    expect(actionUses(workflow)).toEqual([
      { major: 7, repository: "actions/checkout" },
      { major: 4, repository: "github/codeql-action" },
      { major: 5, repository: "googleapis/release-please-action" },
    ]);
  });
});

describe("hasMajor", () => {
  it("finds a major version written vN, vN.x or vN.x.y", () => {
    expect(hasMajor(["v8"], 8)).toBe(true);
    expect(hasMajor(["v8.0.1"], 8)).toBe(true);
  });

  it("does not take v80 for v8", () => {
    expect(hasMajor(["v80.1.0", "v7.9.9"], 8)).toBe(false);
  });
});

describe("latestLtsMajor", () => {
  it("skips the current release and reads the first LTS one", () => {
    const releases = [
      { lts: false, version: "v26.11.1" },
      { lts: "Krypton", version: "v24.21.0" },
      { lts: "Jod", version: "v22.20.0" },
    ];

    expect(latestLtsMajor(releases)).toBe(24);
  });

  it("gives NaN when the index is not a list: the check then fails", () => {
    expect(latestLtsMajor({ error: "rate limited" })).toBeNaN();
  });
});

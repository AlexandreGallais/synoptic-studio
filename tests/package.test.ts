import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Shape of the `package.json` fields checked here. */
type PackageManifest = {
  readonly dependencies?: Readonly<Record<string, string>>;
  readonly devDependencies?: Readonly<Record<string, string>>;
  readonly files?: readonly string[];
  readonly overrides?: Readonly<Record<string, string>>;
};

/** Installed-packages part of docs/tooling/dependencies.md (before "Considered for later"). */
const DEPENDENCIES_PAGE =
  readFileSync(join(import.meta.dirname, "..", "docs", "tooling", "dependencies.md"), "utf8").split(
    "## Considered for later",
    1,
  )[0] ?? "";

/**
 * Parsed `package.json` of the repository.
 *
 * @returns dependency fields of the manifest
 */
function manifest(): PackageManifest {
  const parsed: unknown = JSON.parse(
    readFileSync(join(import.meta.dirname, "..", "package.json"), "utf8"),
  );

  return typeof parsed === "object" && parsed !== null ? parsed : {};
}

/**
 * Package names listed in the first column of the documentation tables.
 *
 * @returns names written as `| \`name\` |` rows
 */
function documentedPackages(): string[] {
  return DEPENDENCIES_PAGE.matchAll(/^\| `(?<name>[^`]+)`\s*\|/gmu)
    .map((match) => match.groups?.["name"] ?? "")
    .toArray();
}

/** Inter's copyright notice and the SIL Open Font License, beside the default font's metrics. */
const INTER_LICENSE = "src/model/font/inter-license.txt";

describe("package.json", () => {
  it("ships the licence of the default font's metrics, which NOTICE names", () => {
    const notice = readFileSync(join(import.meta.dirname, "..", "NOTICE"), "utf8");

    expect(manifest().files).toContain(INTER_LICENSE);
    expect(notice).toContain(INTER_LICENSE);
  });

  it("has no runtime dependency", () => {
    expect(Object.keys(manifest().dependencies ?? {})).toEqual([]);
  });

  it("documents every development dependency in docs/tooling/dependencies.md", () => {
    const documented = new Set(documentedPackages());

    expect(
      Object.keys(manifest().devDependencies ?? {}).filter((name) => !documented.has(name)),
    ).toEqual([]);
  });

  it("documents only installed packages and overrides", () => {
    const installed = new Set([
      ...Object.keys(manifest().devDependencies ?? {}),
      "deepmerge-ts → ^8.0.0",
    ]);

    expect(documentedPackages().filter((name) => !installed.has(name))).toEqual([]);
  });
});

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

/** docs/tooling/dependencies.md. */
const DEPENDENCIES_TEXT = readFileSync(
  join(import.meta.dirname, "..", "docs", "tooling", "dependencies.md"),
  "utf8",
);

/** Installed-packages part of docs/tooling/dependencies.md (before "Crates"). */
const DEPENDENCIES_PAGE = DEPENDENCIES_TEXT.split("## Crates", 1)[0] ?? "";

/** Crates part of docs/tooling/dependencies.md (from "Crates" to "Considered for later"). */
const CRATES_PAGE =
  DEPENDENCIES_TEXT.split("## Crates", 2)[1]?.split("## Considered for later", 1)[0] ?? "";

/** The root Cargo.toml, whose `[workspace.dependencies]` lists the crates of crates.io. */
const CARGO_MANIFEST = readFileSync(join(import.meta.dirname, "..", "Cargo.toml"), "utf8");

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
  return documentedNames(DEPENDENCIES_PAGE);
}

/**
 * Names written in the first column of the tables of a part of the page.
 *
 * @param page - part of docs/tooling/dependencies.md
 * @returns names written as `| \`name\` |` rows
 */
function documentedNames(page: string): string[] {
  return page
    .matchAll(/^\| `(?<name>[^`]+)`\s*\|/gmu)
    .map((match) => match.groups?.["name"] ?? "")
    .toArray();
}

/** Inter's copyright notice and the SIL Open Font License, beside the default font's metrics. */
const INTER_LICENSE = "src/model/font/inter-license.txt";

/**
 * Crates of the root Cargo.toml's `[workspace.dependencies]`, each pinned as `name = "=x.y.z"`.
 *
 * @returns their names, in the manifest's order
 */
function cargoCrates(): string[] {
  const table = CARGO_MANIFEST.split("[workspace.dependencies]", 2)[1]?.split(/^\[/mu, 1)[0] ?? "";

  return table
    .matchAll(/^(?<name>[\w-]+)\s*=/gmu)
    .map((match) => match.groups?.["name"] ?? "")
    .toArray();
}

describe("Cargo.toml", () => {
  it("documents every crate of crates.io, and only those, under Crates", () => {
    expect(new Set(documentedNames(CRATES_PAGE))).toEqual(new Set(cargoCrates()));
  });

  it("pins every crate to an exact version", () => {
    const table = CARGO_MANIFEST.split("[workspace.dependencies]", 2)[1] ?? "";

    expect(table.match(/^[\w-]+\s*=\s*"(?!=)/gmu) ?? []).toEqual([]);
  });
});

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

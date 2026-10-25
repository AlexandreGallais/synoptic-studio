import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Shape of the `package.json` fields checked here. */
type PackageManifest = {
  readonly dependencies?: Readonly<Record<string, string>>;
  readonly devDependencies?: Readonly<Record<string, string>>;
  readonly files?: readonly string[];
  readonly overrides?: Readonly<Record<string, string>>;
};

/** A dependency of the records' `dependencies`: its name, kind and version. */
type Dependency = { readonly kind: string; readonly name: string; readonly version: string };

/**
 * A field of a parsed row, as text.
 *
 * @param row - a parsed row
 * @param key - the field's name
 * @returns its text, or an empty text
 */
function textField(row: unknown, key: string): string {
  const value: unknown =
    typeof row === "object" && row !== null ? Reflect.get(row, key) : undefined;

  return typeof value === "string" ? value : "";
}

/**
 * The dependencies of the records, written by `target/release/knowledge export-data` into
 * `tests/data/dependencies.json`: the CI reads this file, never the records (AUD-054).
 */
const DEPENDENCIES: readonly Dependency[] = (() => {
  const parsed: unknown = JSON.parse(
    readFileSync(join(import.meta.dirname, "data", "dependencies.json"), "utf8"),
  );

  return (Array.isArray(parsed) ? parsed : []).map((row: unknown) => ({
    kind: textField(row, "kind"),
    name: textField(row, "name"),
    version: textField(row, "version"),
  }));
})();

/** The packages a Rust tool of the workspace replaced in E17 (F21.AC5, EN-108 to EN-113). */
const REPLACED_PACKAGE =
  /eslint|prettier|husky|lint-staged|commitlint|secretlint|knip|npm-check-updates|release-please/u;

/** The configuration files of those packages, at the repository's root. */
const REPLACED_CONFIGURATIONS = [
  ".husky",
  ".lintstagedrc.json",
  ".prettierrc.json",
  ".prettierignore",
  ".secretlintrc.json",
  "commitlint.config.js",
  "eslint.config.ts",
  "knip.json",
  "release-please-config.json",
  ".release-please-manifest.json",
];

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
 * The order of two texts, for a sort.
 *
 * @param first - a text
 * @param second - another text
 * @returns negative, zero or positive as `localeCompare` gives it
 */
function byText(first: string, second: string): number {
  return first.localeCompare(second);
}

/**
 * The dependencies of a kind, each `name@version`.
 *
 * @param kind - `npm`, `crate`, `action`…
 * @returns each row of that kind, sorted
 */
function documented(kind: string): readonly string[] {
  return DEPENDENCIES.filter((row) => row.kind === kind)
    .map((row) => `${row.name}@${row.version}`)
    .toSorted(byText);
}

/** Inter's copyright notice and the SIL Open Font License, beside the default font's metrics. */
const INTER_LICENSE = "src/model/font/inter-license.txt";

/**
 * Crates of the root Cargo.toml's `[workspace.dependencies]`, each pinned as `name = "=x.y.z"`.
 *
 * @returns each `name@x.y.z`, sorted
 */
function cargoCrates(): readonly string[] {
  const table = CARGO_MANIFEST.split("[workspace.dependencies]", 2)[1]?.split(/^\[/mu, 1)[0] ?? "";

  return table
    .matchAll(/^(?<name>[\w-]+)\s*=\s*(?:\{\s*version\s*=\s*)?"=(?<version>[^"]+)"/gmu)
    .map((match) => Object.values(match.groups ?? {}).join("@"))
    .toArray()
    .toSorted(byText);
}

/**
 * The Actions the workflows use, each `owner/name@major`, a sub-action read as its repository's.
 *
 * @returns each once, sorted
 */
function workflowActions(): readonly string[] {
  const workflows = join(import.meta.dirname, "..", ".github", "workflows");
  const uses = readdirSync(workflows)
    .flatMap((name) =>
      readFileSync(join(workflows, name), "utf8")
        .matchAll(/uses: (?<repository>[\w.-]+\/[\w.-]+)(?:\/[\w./-]+)?@(?<version>\S+)/gu)
        .map((match) => Object.values(match.groups ?? {}).join("@"))
        .toArray(),
    )
    .toSorted(byText);

  return [...new Set(uses)];
}

describe("Cargo.toml", () => {
  it("documents every crate of crates.io, and only those, at its version", () => {
    expect(documented("crate")).toEqual(cargoCrates());
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

  it("[F21.AC5] keeps no package that a Rust tool replaced, nor its configuration nor its workflow step", () => {
    const root = join(import.meta.dirname, "..");
    const workflows = join(root, ".github", "workflows");
    const steps = readdirSync(workflows)
      .map((name) => readFileSync(join(workflows, name), "utf8"))
      .join("\n");
    const packages = [
      ...Object.keys(manifest().devDependencies ?? {}),
      ...DEPENDENCIES.map((row) => row.name),
    ].filter((name) => REPLACED_PACKAGE.test(name));
    const configurations = REPLACED_CONFIGURATIONS.filter((name) => existsSync(join(root, name)));

    expect(packages).toEqual([]);
    expect(configurations).toEqual([]);
    expect(REPLACED_PACKAGE.exec(steps) ?? []).toEqual([]);
  });

  it("documents every development dependency, and only those, at its version", () => {
    const installed = Object.entries(manifest().devDependencies ?? {})
      .map(([name, version]) => `${name}@${version}`)
      .toSorted(byText);

    expect(documented("npm")).toEqual(installed);
  });
});

describe("the workflows", () => {
  it("document every Action they use, and only those, at its major", () => {
    expect(documented("action")).toEqual(workflowActions());
  });
});

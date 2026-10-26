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

/** A dependency of the records' `dependencies`: its project, name, kind and version. */
type Dependency = {
  readonly kind: string;
  readonly name: string;
  readonly project: string;
  readonly version: string;
};

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
    project: textField(row, "project"),
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
 * The root project's dependencies of a kind, each `name@version`: the playground's are its own
 * project's (ADR-0104).
 *
 * @param kind - `npm`, `crate`, `action`…
 * @returns each root row of that kind, sorted
 */
function documented(kind: string): readonly string[] {
  return DEPENDENCIES.filter((row) => row.project === "root" && row.kind === kind)
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

/** The root's scripts: ADR-0103's list and the playground's, which ADR-0104 adds. */
const ROOT_SCRIPTS = ["check:all", "dev", "docs:dev", "fix", "playground", "prepare", "test"];

/**
 * A manifest's field, read without a type: `workspaces` and `scripts` are checked here alone.
 *
 * @param path - the manifest's path from the repository's root
 * @param key - the field's name
 * @returns the field's value, or `undefined`
 */
function manifestField(path: string, key: string): unknown {
  const parsed: unknown = JSON.parse(readFileSync(join(import.meta.dirname, "..", path), "utf8"));

  return typeof parsed === "object" && parsed !== null ? Reflect.get(parsed, key) : undefined;
}

/**
 * The keys of a manifest's field holding an object.
 *
 * @param path - the manifest's path from the repository's root
 * @param key - the field's name
 * @returns its keys, sorted, or none when it holds no object
 */
function manifestKeys(path: string, key: string): readonly string[] {
  const field = manifestField(path, key);

  return typeof field === "object" && field !== null ? Object.keys(field).toSorted(byText) : [];
}

describe("the two npm projects", () => {
  it("[F50.AC1] keeps the playground a project of its own beside the root's", () => {
    const installed = manifestKeys("playground/package-lock.json", "packages");

    // Neither manifest is a workspace: the playground has its own lockfile, Angular zoneless.
    expect(manifestField("package.json", "workspaces")).toBeUndefined();
    expect(manifestField("playground/package.json", "workspaces")).toBeUndefined();
    expect(installed).toContain("node_modules/@angular/core");
    expect(installed.filter((name) => name.endsWith("/zone.js"))).toEqual([]);
    // The root keeps TypeScript 7.0.2 and gains the script `playground` alone.
    expect(manifest().devDependencies?.["typescript"]).toBe("7.0.2");
    expect(manifestKeys("package.json", "scripts")).toEqual(ROOT_SCRIPTS);
  });
});

/**
 * The playground's `package.json`, an npm project of its own (ADR-0104).
 *
 * @returns its dependency fields
 */
function playgroundManifest(): PackageManifest {
  const parsed: unknown = JSON.parse(
    readFileSync(join(import.meta.dirname, "..", "playground", "package.json"), "utf8"),
  );

  return typeof parsed === "object" && parsed !== null ? parsed : {};
}

describe("playground/package.json", () => {
  it("[F50.AC1] documents every package of the playground, and only those, at its version", () => {
    const { dependencies, devDependencies } = playgroundManifest();
    const installed = Object.entries({ ...dependencies, ...devDependencies })
      .map(([name, version]) => `${name}@${version}`)
      .toSorted(byText);
    const rows = DEPENDENCIES.filter((row) => row.project === "playground" && row.kind === "npm")
      .map((row) => `${row.name}@${row.version}`)
      .toSorted(byText);

    expect(rows).toEqual(installed);
  });
});

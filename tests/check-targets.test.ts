import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** Time limit of a test asking a tool about every tracked file: about 4.3 s alone (EN-054), seven times. */
const WALK_TIMEOUT = 30_000;

/** The start of a script run under the heavy lock (EN-047), a crate of the workspace (EN-068). */
const HEAVY_LOCK = /^cargo run --release -q -p heavy-lock -- /u;

/** The command of the repository's formatter, oxfmt then the tables without padding (EN-073). */
const FORMAT = "cargo run --release -q -p format --";

/** The command of the project's linter, run after oxlint (EN-075). */
const PROJECT_LINT = "cargo run --release -q -p lint --";

/** The Markdown pages the project's linter leaves out: the changelog, written by release-please. */
const UNLINTED_PAGES = new Set(["CHANGELOG.md"]);

/** oxfmt, as npm installs it. */
const OXFMT = join(ROOT, "node_modules", ".bin", "oxfmt");

/**
 * Whether a parsed JSON value is an object.
 *
 * @param value - any parsed value
 * @returns true for an object, `null` left out
 */
function isObject(value: unknown): value is object {
  return typeof value === "object" && value !== null;
}

/**
 * The `scripts` field of a parsed manifest.
 *
 * @param manifest - `package.json`, parsed
 * @returns the field, or an empty object when it is missing
 */
function scriptsField(manifest: unknown): object {
  const field: unknown = isObject(manifest) && "scripts" in manifest ? manifest.scripts : {};

  return isObject(field) ? field : {};
}

/**
 * Scripts of `package.json`, by name.
 *
 * @returns each script's command
 */
function readScripts(): ReadonlyMap<string, string> {
  const parsed: unknown = JSON.parse(readFileSync(join(ROOT, "package.json"), "utf8"));
  const entries = Object.entries(scriptsField(parsed));

  return new Map(entries.map(([name, command]): [string, string] => [name, String(command)]));
}

/** Scripts of `package.json`. */
const SCRIPTS = readScripts();

/** Files tracked by Git. */
const TRACKED = execFileSync("git", ["ls-files"], { cwd: ROOT, encoding: "utf8" })
  .split("\n")
  .filter((path) => path !== "");

/**
 * Targets given to a tool in one command of a script: its arguments that are not options.
 *
 * @param script - the npm script's name
 * @param tool - the command's start (`oxlint`, the formatter's `cargo run … --`)
 * @returns its paths and globs, unquoted
 */
function targetsOf(script: string, tool: string): readonly string[] {
  const command = (SCRIPTS.get(script) ?? "")
    .split("&&")
    .map((part) => part.trim())
    .find((part) => part.startsWith(`${tool} `));
  const words = (command ?? "").slice(tool.length).trim().split(" ");

  // An option's value follows it: `--cache-location <folder>`.
  return words
    .filter(
      (word, index) =>
        word !== "" && !word.startsWith("-") && words[index - 1] !== "--cache-location",
    )
    .map((word) => word.replaceAll('"', ""));
}

/**
 * Endings a root glob asks for: `.ts` and `.md` for `*.{ts,md}`, any ending for `*`.
 *
 * @param glob - `*` or `*.{ts,md}`
 * @returns the endings, `""` matching every file
 */
function globEndings(glob: string): readonly string[] {
  const list = /^\*\.\{(?<list>[^}]+)\}$/u.exec(glob)?.groups?.["list"] ?? "";

  return list === "" ? [""] : list.split(",").map((extension) => `.${extension}`);
}

/**
 * Whether a root glob covers a tracked file: a root file with one of its endings.
 *
 * @param glob - `*` or `*.{ts,md}`
 * @param path - a tracked file, from the root
 * @returns true when the glob matches the file
 */
function globCovers(glob: string, path: string): boolean {
  return !path.includes("/") && globEndings(glob).some((ending) => path.endsWith(ending));
}

/**
 * Whether a target covers a tracked file: the file, a folder holding it, or a root glob.
 *
 * @param target - a path or a root glob (`*`, `*.{ts,md}`)
 * @param path - a tracked file, from the root
 * @returns true when the tool reads the file through this target
 */
function covers(target: string, path: string): boolean {
  return target.startsWith("*")
    ? globCovers(target, path)
    : path === target || path.startsWith(`${target}/`);
}

/**
 * Tracked files that a tool should read but none of its targets covers.
 *
 * @param targets - the tool's targets
 * @param candidates - the tracked files the tool checks when it is given everything
 * @returns the files left out
 */
function leftOut(targets: readonly string[], candidates: readonly string[]): readonly string[] {
  return candidates.filter((path) => targets.every((target) => !covers(target, path)));
}

/**
 * The crates of `tools/`: each folder holding a `Cargo.toml`.
 *
 * @returns their folder names, sorted
 */
function crateNames(): readonly string[] {
  return readdirSync(join(ROOT, "tools"), { withFileTypes: true })
    .filter(
      (entry) => entry.isDirectory() && existsSync(join(ROOT, "tools", entry.name, "Cargo.toml")),
    )
    .map((entry) => entry.name)
    .toSorted((left, right) => left.localeCompare(right));
}

/**
 * Number of files oxfmt reads from some paths, the root's files for `*` as the formatter reads it.
 *
 * @param targets - the formatter's targets, none for the whole repository
 * @returns the count oxfmt prints, `Finished in … on 2111 files`
 */
function oxfmtCount(targets: readonly string[]): number {
  const paths = targets.flatMap((target) =>
    target === "*"
      ? readdirSync(ROOT).filter((name) => statSync(join(ROOT, name)).isFile())
      : target,
  );
  const run = spawnSync(OXFMT, ["--check", "--no-error-on-unmatched-pattern", ...paths], {
    cwd: ROOT,
    encoding: "utf8",
  });
  const count = /on (?<count>\d+) files/u.exec(run.stdout)?.groups?.["count"] ?? "0";

  return Number(count);
}

describe("check tools given explicit targets (EN-045)", () => {
  it("[F18.AC20] gives the formatter the same targets in format, format:check and fix", () => {
    const targets = targetsOf("format:check", FORMAT);

    expect(targets.length).toBeGreaterThan(0);
    expect(targetsOf("format", FORMAT)).toEqual(targets);
    expect(targetsOf("fix", FORMAT)).toEqual(targets);
  });

  it("[F19.AC5] formats with the formatter only, staged files included, never Prettier", () => {
    const staged = readFileSync(join(ROOT, ".lintstagedrc.json"), "utf8");
    const scripts = SCRIPTS.values().toArray().join("\n");

    expect(staged).toContain(`"${FORMAT} --write"`);
    expect(`${staged}\n${scripts}`).not.toMatch(/\bprettier /u);
    expect(existsSync(join(ROOT, ".prettierrc.json"))).toBe(false);
  });

  it("[F19.AC1] gives oxlint the same targets in lint and fix, every tracked TypeScript file", () => {
    const targets = targetsOf("lint", "oxlint");
    const sources = TRACKED.filter((path) => path.endsWith(".ts"));

    expect(targets.length).toBeGreaterThan(0);
    expect(targetsOf("fix", "oxlint")).toEqual(targets);

    // The Rust crates hold TypeScript only as their tests' fixtures, which no linter reads.
    expect(
      leftOut(targets, sources).filter((path) => !/^tools\/[^/]+\/tests\/fixtures\//u.test(path)),
    ).toEqual([]);
  });

  it("[F19.AC3] gives the project's linter oxlint's targets in lint and fix, after oxlint", () => {
    const targets = targetsOf("lint", "oxlint");

    for (const script of ["lint", "fix"]) {
      const commands = (SCRIPTS.get(script) ?? "").split("&&").map((part) => part.trim());

      expect(targetsOf(script, PROJECT_LINT)).toEqual(targetsOf("lint", PROJECT_LINT));

      expect(targets.filter((target) => !targetsOf(script, PROJECT_LINT).includes(target))).toEqual(
        [],
      );

      expect(commands.findIndex((part) => part.startsWith(PROJECT_LINT))).toBe(
        commands.findIndex((part) => part.startsWith("oxlint ")) + 1,
      );
    }
  });

  it("[F19.AC4] gives the project's linter every tracked Markdown page", () => {
    const pages = TRACKED.filter((path) => path.endsWith(".md") && !UNLINTED_PAGES.has(path));
    const staged: unknown = JSON.parse(readFileSync(join(ROOT, ".lintstagedrc.json"), "utf8"));

    expect(pages.length).toBeGreaterThan(0);
    expect(leftOut(targetsOf("lint", PROJECT_LINT), pages)).toEqual([]);
    expect(staged).toHaveProperty(["*.md"], [`${FORMAT} --write`, `${PROJECT_LINT} --fix`]);
  });

  it(
    "[F19.AC5] leaves no file that oxfmt formats out of the formatter's targets",
    { timeout: WALK_TIMEOUT },
    () => {
      const named = oxfmtCount(targetsOf("format:check", FORMAT));

      expect(named).toBeGreaterThan(0);
      expect(named).toBe(oxfmtCount([]));
    },
  );

  it("[F18.AC20] runs secretlint on the tracked files only", () => {
    expect(SCRIPTS.get("lint:secrets")).toMatch(/^git ls-files -z \| xargs -0 secretlint /u);
  });
});

describe("heavy commands and Rust tools (AUD-020)", () => {
  it.each(["test", "check:all", "golden:check", "docs:build", "mutate"])(
    "[F18.AC3] runs %s through the heavy lock",
    (script) => {
      expect(SCRIPTS.get(script)).toMatch(HEAVY_LOCK);
    },
  );

  it("[F18.AC7] merges stories one at a time under the merge lock", () => {
    expect(SCRIPTS.get("story:finish")).toMatch(
      new RegExp(String.raw`${HEAVY_LOCK.source}--lock merge node scripts/story-finish\.ts$`, "u"),
    );
  });

  it("[F19.AC9] checks the whole Rust workspace in check:rust, its binaries built before the lanes", () => {
    expect(SCRIPTS.get("check:rust")?.split(" && ")).toEqual([
      "cargo fmt --all --check",
      "cargo clippy --workspace --all-targets --locked --quiet",
      "npm run --silent rust:advisories",
      "npm run --silent rust:coverage",
      "cargo check --workspace --all-targets --locked --quiet --target x86_64-pc-windows-msvc",
    ]);
  });

  it("[F19.AC10] installs the Windows target its check compiles for without linking", () => {
    expect(readFileSync(join(ROOT, "rust-toolchain.toml"), "utf8")).toMatch(
      /^targets = \["x86_64-pc-windows-msvc"\]$/mu,
    );
  });

  it("[F19.AC9] checks the crates against the RustSec advisories offline, failing aloud without the tool", () => {
    const advisories = SCRIPTS.get("rust:advisories") ?? "";

    expect(advisories).toMatch(
      /^cargo deny --version > \/dev\/null 2>&1 \|\| \{ echo '[^']*cargo install --locked cargo-deny@0\.20\.2[^']*' >&2; exit 1; \}; /u,
    );

    expect(advisories).toContain(
      "cargo deny --offline check advisories || { echo 'A Rust crate has a RustSec advisory, or the advisory database is missing: cargo deny fetch db",
    );
  });

  it("[F19.AC9] fetches the advisory database in the CI before check:all", () => {
    const workflow = readFileSync(join(ROOT, ".github", "workflows", "ci.yml"), "utf8");

    expect(workflow).toMatch(
      /- run: cargo install --locked cargo-llvm-cov@0\.9\.1 cargo-deny@0\.20\.2\n {6}- run: cargo deny fetch db\n {6}- run: npm run check:all\n/u,
    );
  });

  it("[F19.AC9] runs the workspace's tests under a floor of lines and functions, failing aloud without the tool", () => {
    const coverage = SCRIPTS.get("rust:coverage") ?? "";

    expect(coverage).toMatch(
      /^cargo llvm-cov --version > \/dev\/null 2>&1 \|\| \{ echo '[^']*cargo install --locked cargo-llvm-cov@0\.9\.1[^']*' >&2; exit 1; \}; /u,
    );

    expect(coverage).toContain(
      "cargo llvm-cov --workspace --locked --quiet --summary-only --fail-under-lines 93 --fail-under-functions 93 || { echo 'Rust tests failed, or their coverage fell under the floor of 93 % of lines and 93 % of functions",
    );
  });
});

describe("the Rust workspace at the root (EN-068)", () => {
  const manifest = readFileSync(join(ROOT, "Cargo.toml"), "utf8");

  it("[F19.AC9] gathers tools/* in a virtual workspace sharing its settings and lints", () => {
    expect(manifest).not.toMatch(/^\[package\]$/mu);
    expect(manifest).toMatch(/^members = \["tools\/\*"\]$/mu);
    expect(manifest).toMatch(/^resolver = "3"$/mu);

    expect(manifest).toMatch(
      /^\[workspace\.package\]\nedition = "2024"\nlicense = "Apache-2\.0"\npublish = false$/mu,
    );

    expect(manifest).toMatch(/^\[workspace\.lints\.rust\]\nunsafe_code = "forbid"$/mu);
    expect(manifest).toMatch(/^pedantic = \{ level = "warn", priority = -1 \}$/mu);
    expect(manifest).toMatch(/^\[workspace\.dependencies\]$/mu);
  });

  it("[F19.AC9] pins the toolchain and denies warnings", () => {
    expect(readFileSync(join(ROOT, "rust-toolchain.toml"), "utf8")).toMatch(
      /^channel = "1\.99\.1"\ncomponents = \["rustfmt", "clippy", "llvm-tools"\]$/mu,
    );

    expect(readFileSync(join(ROOT, ".cargo", "config.toml"), "utf8")).toMatch(
      /^\[build\]\nwarnings = "deny"$/mu,
    );
  });

  it.each(crateNames())("[F19.AC9] makes %s a member inheriting the workspace's lints", (name) => {
    const crate = readFileSync(join(ROOT, "tools", name, "Cargo.toml"), "utf8");

    expect(crate).toMatch(/^\[lints\]\nworkspace = true$/mu);
    expect(crate).toMatch(/^edition\.workspace = true$/mu);
    expect(existsSync(join(ROOT, "tools", name, "Cargo.lock"))).toBe(false);
  });

  it("[F19.AC9] leaves the build output target/ to Git, oxlint, the formatter and secretlint", () => {
    expect(readFileSync(join(ROOT, ".gitignore"), "utf8")).toMatch(/^\/target\/$/mu);
    expect(readFileSync(join(ROOT, ".secretlintignore"), "utf8")).toMatch(/^target$/mu);
    expect(readFileSync(join(ROOT, ".oxlintrc.json"), "utf8")).toContain('"target/**"');
    expect(readFileSync(join(ROOT, ".oxfmtrc.json"), "utf8")).toMatch(/^ {4}"target",?$/mu);
  });

  it("[F19.AC9] tracks one Cargo.lock, at the root, and no build output", () => {
    expect(TRACKED.filter((path) => path.endsWith("Cargo.lock"))).toEqual(["Cargo.lock"]);
    expect(TRACKED.filter((path) => /(?:^|\/)target\//u.test(path))).toEqual([]);
  });
});

describe("the Rust tools run as crates of the workspace (EN-068)", () => {
  it("[F19.AC9] runs each Rust tool of a script or a Git hook by its crate, never by its manifest", () => {
    const callers = [
      SCRIPTS.values().toArray().join("\n"),
      readFileSync(join(ROOT, ".husky", "pre-push"), "utf8"),
      readFileSync(join(ROOT, ".husky", "commit-msg"), "utf8"),
    ].join("\n");
    const runs = callers.match(/cargo run(?:(?!cargo run)[^&;\n])*/gu) ?? [];

    expect(runs.length).toBeGreaterThan(0);
    expect(runs.filter((run) => !run.startsWith("cargo run --release -q -p "))).toEqual([]);
    expect(callers).not.toMatch(/manifest-path/u);
  });

  it("[F19.AC9] builds the release binaries of the workspace from prepare", () => {
    expect(SCRIPTS.get("rust:build")).toBe("cargo build --release --workspace --locked");
    expect(SCRIPTS.get("prepare")).toBe("husky && npm run rust:build");
  });
});

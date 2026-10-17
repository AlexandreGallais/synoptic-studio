import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { ESLint } from "eslint";
import { getFileInfo } from "prettier";
import { describe, expect, it } from "vitest";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** The project's ESLint, read from its configuration. */
const LINT = new ESLint({ cwd: ROOT });

/** Time limit of a test asking a tool about every tracked file: about 4.3 s alone (EN-054), seven times. */
const WALK_TIMEOUT = 30_000;

/** The start of a script run under the heavy lock (EN-047), from its release binary (EN-068). */
const HEAVY_LOCK = /^scripts\/rust-tool\.sh heavy-lock /u;

/** The launcher of the Rust tools' release binaries (EN-068). */
const LAUNCHER = join(ROOT, "scripts", "rust-tool.sh");

/** Prettier's ignore files, as its command line reads them. */
const PRETTIER_IGNORE = [join(ROOT, ".gitignore"), join(ROOT, ".prettierignore")];

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
 * @param tool - the command's first word (`prettier`, `eslint`)
 * @returns its paths and globs, unquoted
 */
function targetsOf(script: string, tool: string): readonly string[] {
  const command = (SCRIPTS.get(script) ?? "")
    .split("&&")
    .map((part) => part.trim())
    .find((part) => part.startsWith(`${tool} `));
  const words = (command ?? "").split(" ").slice(1);

  // An option's value follows it: `--cache-location <folder>`.
  return words
    .filter((word, index) => !word.startsWith("-") && words[index - 1] !== "--cache-location")
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

describe("check tools given explicit targets (EN-045)", () => {
  it("[F18.AC20] gives Prettier the same targets in format, format:check and fix", () => {
    const targets = targetsOf("format:check", "prettier");

    expect(targets.length).toBeGreaterThan(0);
    expect(targetsOf("format", "prettier")).toEqual(targets);
    expect(targetsOf("fix", "prettier")).toEqual(targets);
  });

  it("[F18.AC20] gives ESLint the same targets in lint, lint:fix and fix", () => {
    const targets = targetsOf("lint", "eslint");

    expect(targets.length).toBeGreaterThan(0);
    expect(targetsOf("lint:fix", "eslint")).toEqual(targets);
    expect(targetsOf("fix", "eslint")).toEqual(targets);
  });

  it(
    "[F18.AC20] leaves no tracked file that ESLint checks out of its targets",
    { timeout: WALK_TIMEOUT },
    async () => {
      const linted = await Promise.all(
        TRACKED.filter((path) => /\.(?:ts|md)$/u.test(path)).map(async (path) =>
          (await LINT.isPathIgnored(join(ROOT, path))) ? "" : path,
        ),
      );

      expect(
        leftOut(
          targetsOf("lint", "eslint"),
          linted.filter((path) => path !== ""),
        ),
      ).toEqual([]);
    },
  );

  it(
    "[F18.AC20] leaves no tracked file that Prettier formats out of its targets",
    { timeout: WALK_TIMEOUT },
    async () => {
      const formatted = await Promise.all(
        TRACKED.map(async (path) => {
          const fileInfo = await getFileInfo(join(ROOT, path), { ignorePath: PRETTIER_IGNORE });

          return fileInfo.ignored || fileInfo.inferredParser === null ? "" : path;
        }),
      );

      expect(
        leftOut(
          targetsOf("format:check", "prettier"),
          formatted.filter((path) => path !== ""),
        ),
      ).toEqual([]);
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

  it("[F19.AC9] checks the whole Rust workspace in check:rust, then builds its binaries", () => {
    expect(SCRIPTS.get("check:rust")?.split(" && ")).toEqual([
      "cargo fmt --all --check",
      "cargo clippy --workspace --all-targets --locked --quiet",
      "cargo test --workspace --locked --quiet",
      "npm run --silent rust:build",
    ]);
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
      /^channel = "1\.99\.1"\ncomponents = \["rustfmt", "clippy"\]$/mu,
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

  it("[F19.AC9] leaves the build output target/ to Git, ESLint, Prettier and secretlint", async () => {
    const built = join(ROOT, "target", "release", "build", "page.md");

    expect(readFileSync(join(ROOT, ".gitignore"), "utf8")).toMatch(/^\/target\/$/mu);
    expect(readFileSync(join(ROOT, ".secretlintignore"), "utf8")).toMatch(/^target$/mu);
    expect(await LINT.isPathIgnored(built)).toBe(true);

    const formatted = await getFileInfo(built, { ignorePath: PRETTIER_IGNORE });

    expect(formatted.ignored).toBe(true);
  });

  it("[F19.AC9] tracks one Cargo.lock, at the root, and no build output", () => {
    expect(TRACKED.filter((path) => path.endsWith("Cargo.lock"))).toEqual(["Cargo.lock"]);
    expect(TRACKED.filter((path) => /(?:^|\/)target\//u.test(path))).toEqual([]);
  });
});

describe("the Rust tools run from their release binaries (EN-068)", () => {
  it("[F19.AC9] runs no Rust tool through cargo run in a script, a Git hook or a Claude Code hook", () => {
    const callers = [
      SCRIPTS.values().toArray().join("\n"),
      readFileSync(join(ROOT, ".husky", "pre-push"), "utf8"),
      readFileSync(join(ROOT, ".husky", "commit-msg"), "utf8"),
      readFileSync(join(ROOT, ".claude", "settings.json"), "utf8"),
      readFileSync(join(ROOT, "scripts", "story-finish-rows.ts"), "utf8"),
    ].join("\n");

    expect(callers).not.toMatch(/cargo run|"run",\s*"--release"/u);
  });

  it("[F19.AC9] builds the release binaries of the workspace from prepare", () => {
    expect(SCRIPTS.get("rust:build")).toBe("cargo build --release --workspace --locked");
    expect(SCRIPTS.get("prepare")).toBe("husky && npm run rust:build");
  });

  it("fails naming the build command when a tool's binary is missing (EN-068)", () => {
    const result = spawnSync(LAUNCHER, ["no-such-tool", "--flag"], { encoding: "utf8" });

    expect(result.status).toBe(127);

    expect(result.stderr).toBe(
      "target/release/no-such-tool is missing: build the Rust tools with `npm run rust:build`\n",
    );
  });

  it("[F19.AC9] runs a tool's release binary with its arguments and its exit code", () => {
    const result = spawnSync(LAUNCHER, ["backlog-index"], { encoding: "utf8" });

    expect(result.status).toBe(2);
    expect(result.stderr).toMatch(/usage/iu);
  });
});

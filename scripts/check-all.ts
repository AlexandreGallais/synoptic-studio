// Temporal runs behind `--harmony-temporal` in Node.js 24: the npm script passes it.

// Every check of `npm run check:all`, in parallel lanes (RET-002): the checks of one lane run in
// order — the playground is built into the documentation site, after it — and the lanes at the
// same time, so that the pre-push hook waits for the slowest lane instead of the sum (about 80 s
// one after the other). Each check prints a line with its time; a failed check prints its output.
//
// No release binary is written while the lanes run (EN-069's fix): `cargo run` copies its crate's
// binary into target/release/ again even when it is up to date, removing it first, so a lane
// starting that binary at the same moment failed with `could not execute process … No such file or
// directory`. The release binaries are built once before the lanes, and a lane runs a script that
// takes the heavy lock without it, as check:all already holds that lock.

import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

/** Lanes of npm scripts: in order within a lane, the lanes at the same time. */
export const LANES: readonly (readonly string[])[] = [
  ["lint"],
  ["test"],
  ["docs:build", "playground:build"],
  ["format:check", "lint:secrets", "deps:audit", "deps:unused", "deps:tools", "deps:outdated"],
  // The Rust workspace: rustfmt, Clippy, its advisories, its tests (EN-068, EN-069).
  ["check:rust"],
];

/** The check run alone before the lanes: the release binaries every lane's tools run from. */
export const BEFORE_LANES = "rust:build";

/** How a script starts when it takes the heavy lock, which check:all holds for its lanes. */
const HEAVY_LOCK = "cargo run --release -q -p heavy-lock -- ";

/**
 * The command a lane runs for a script: the script itself, without the heavy lock check:all
 * already holds, so that no two lanes start `cargo run -p heavy-lock` at once.
 *
 * @param script - an npm script's command
 * @returns what follows `cargo run --release -q -p heavy-lock --`, any other script unchanged
 */
export function commandOf(script: string): string {
  return script.startsWith(HEAVY_LOCK) ? script.slice(HEAVY_LOCK.length) : script;
}

/**
 * The npm scripts of the repository.
 *
 * @returns each script's command, by name
 */
export function readScripts(): ReadonlyMap<string, string> {
  const manifest: unknown = JSON.parse(
    readFileSync(join(import.meta.dirname, "..", "package.json"), "utf8"),
  );
  const field: unknown = isObject(manifest) && "scripts" in manifest ? manifest.scripts : {};
  const entries = isObject(field) ? Object.entries(field) : [];

  return new Map(entries.map(([name, command]): [string, string] => [name, String(command)]));
}

/**
 * Whether a parsed JSON value is an object.
 *
 * @param value - any parsed value
 * @returns true for an object, `null` left out
 */
function isObject(value: unknown): value is object {
  return typeof value === "object" && value !== null;
}

/** Milliseconds per second. */
const SECOND = 1000;

/** The result of one check. */
export type CheckResult = {
  readonly durationMs: number;
  readonly isOk: boolean;
  readonly name: string;
  readonly output: string;
};

/**
 * Line reporting a check, its output after it when it failed.
 *
 * @param result - the check's result
 * @returns `✓ lint 23.9 s`, or `✗ test 26.2 s` followed by its output
 */
export function reportOf(result: CheckResult): string {
  const seconds = (result.durationMs / SECOND).toFixed(1);

  return result.isOk
    ? `✓ ${result.name} ${seconds} s`
    : `✗ ${result.name} ${seconds} s\n${result.output}`;
}

/**
 * Runs one npm script's command in the shell npm runs it in, its output kept; check:all is started
 * by npm, so the command finds the tools of `node_modules/.bin`.
 *
 * @param name - the script
 * @param scripts - the npm scripts, by name
 * @returns its result once it ends
 */
async function runCheck(name: string, scripts: ReadonlyMap<string, string>): Promise<CheckResult> {
  const start = Temporal.Now.instant().epochMilliseconds;
  const child = spawn(commandOf(scripts.get(name) ?? `npm run ${name}`), {
    shell: true,
    stdio: ["ignore", "pipe", "pipe"],
  });
  const chunks: string[] = [];

  child.stdout.on("data", (chunk: Buffer) => {
    chunks.push(chunk.toString());
  });

  child.stderr.on("data", (chunk: Buffer) => {
    chunks.push(chunk.toString());
  });

  const code = await new Promise<number | null>((resolve) => {
    child.on("close", resolve);
  });
  const result = {
    durationMs: Temporal.Now.instant().epochMilliseconds - start,
    isOk: code === 0,
    name,
    output: chunks.join(""),
  };

  process.stdout.write(`${reportOf(result)}\n`);

  return result;
}

/**
 * Runs the checks of a lane one after the other, every one even after a failure.
 *
 * @param lane - npm scripts, in order
 * @param scripts - the npm scripts, by name
 * @returns their results
 */
async function runLane(
  lane: readonly string[],
  scripts: ReadonlyMap<string, string>,
): Promise<readonly CheckResult[]> {
  const results: CheckResult[] = [];

  // Each check starts once the previous one has ended; its result joins a local list.
  await lane.reduce(async (previous, name) => {
    await previous;
    results.push(await runCheck(name, scripts));
  }, Promise.resolve());

  return results;
}

if (import.meta.main) {
  const start = Temporal.Now.instant().epochMilliseconds;
  const scripts = readScripts();
  const built = await runCheck(BEFORE_LANES, scripts);
  // Without the release binaries every lane's `cargo run` would build them at once: none starts.
  const lanes = built.isOk
    ? await Promise.all(LANES.map(async (lane) => await runLane(lane, scripts)))
    : [];
  const failed = [built, ...lanes.flat()]
    .filter((result) => !result.isOk)
    .map((result) => result.name);
  const seconds = ((Temporal.Now.instant().epochMilliseconds - start) / SECOND).toFixed(1);

  process.stdout.write(
    failed.length === 0
      ? `check:all passed in ${seconds} s\n`
      : `check:all failed in ${seconds} s: ${failed.join(", ")}\n`,
  );

  process.exitCode = failed.length === 0 ? 0 : 1;
}

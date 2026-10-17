// Temporal runs behind `--harmony-temporal` in Node.js 24: the npm script passes it.

// Every check of `npm run check:all`, in parallel lanes (RET-002): the checks of one lane run in
// order — the playground is built into the documentation site, after it — and the lanes at the
// same time, so that the pre-push hook waits for the slowest lane instead of the sum (about 80 s
// one after the other). Each check prints a line with its time; a failed check prints its output.

import { spawn } from "node:child_process";

/** Lanes of npm scripts: in order within a lane, the lanes at the same time. */
export const LANES: readonly (readonly string[])[] = [
  ["lint"],
  ["test"],
  ["docs:build", "playground:build"],
  [
    "format:check",
    "lint:secrets",
    "typecheck",
    "deps:audit",
    "deps:unused",
    "deps:tools",
    "deps:outdated",
  ],
  // The Rust workspace: rustfmt, Clippy, its tests, then its release binaries (EN-068).
  ["check:rust"],
];

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
 * Runs one npm script, its output kept.
 *
 * @param name - the script
 * @returns its result once it ends
 */
async function runCheck(name: string): Promise<CheckResult> {
  const start = Temporal.Now.instant().epochMilliseconds;
  const child = spawn("npm", ["run", "--silent", name], { stdio: ["ignore", "pipe", "pipe"] });
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
 * @returns their results
 */
async function runLane(lane: readonly string[]): Promise<readonly CheckResult[]> {
  const results: CheckResult[] = [];

  // Each check starts once the previous one has ended; its result joins a local list.
  await lane.reduce(async (previous, name) => {
    await previous;
    results.push(await runCheck(name));
  }, Promise.resolve());

  return results;
}

if (import.meta.main) {
  const start = Temporal.Now.instant().epochMilliseconds;
  const lanes = await Promise.all(LANES.map(async (lane) => await runLane(lane)));
  const failed = lanes
    .flat()
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

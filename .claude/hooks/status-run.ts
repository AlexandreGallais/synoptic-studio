// The open run's part of the status line (EN-092, docs/tooling/metrics.md): its duration and
// tokens, counted by the Rust tool `run-totals` from the marker `.git/metrics/run.json`.

import { execFileSync } from "node:child_process";

import { field } from "./status-json.ts";

/**
 * A JSON text read, nothing when it is not JSON.
 *
 * @param text - the text to read
 * @returns the value, or `undefined`
 */
function parsedOrNothing(text: string): unknown {
  try {
    return JSON.parse(text);
  } catch {
    return undefined;
  }
}

/**
 * Whether the run's marker holds a run still open (`end` 0), as `run-totals start` writes it.
 *
 * @param marker - text of `.git/metrics/run.json`, empty when missing
 * @returns true while a `/run` goes on
 */
export function isRunOpen(marker: string): boolean {
  const run = parsedOrNothing(marker);

  return field(run, "end") === 0 && typeof field(run, "start") === "number";
}

/**
 * The open run's part of the line, counted by the Rust tool `run-totals` (EN-092); spawned only
 * while a run is open.
 *
 * @param root - the checkout, where Cargo finds the workspace
 * @param marker - text of `.git/metrics/run.json`, empty when missing
 * @returns `run 2 h 05 · 48,123,456 tokens`, empty outside a run or when the tool fails
 */
export function runPart(root: string, marker: string): string {
  if (!isRunOpen(marker)) {
    return "";
  }

  try {
    return execFileSync("cargo", ["run", "--release", "-q", "-p", "run-totals", "--", "status"], {
      cwd: root,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    }).trim();
  } catch {
    return "";
  }
}

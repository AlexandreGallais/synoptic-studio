// PreToolUse hook (docs/tooling/metrics.md): writes the tool call that starts to the Git
// directory, so that the status line shows what is running and for how long; the
// `log-tool-call` hook removes it when the call ends. Runs in the background (`async`).

// Temporal runs behind `--harmony-temporal` in Node.js 24: the hook command passes it.

import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { runningCall } from "./status-place.ts";

if (import.meta.main) {
  const input: unknown = JSON.parse(readFileSync(0, "utf8"));
  const id =
    typeof input === "object" && input !== null ? String(Reflect.get(input, "tool_use_id")) : "";
  const common = execFileSync("git", ["rev-parse", "--path-format=absolute", "--git-common-dir"], {
    encoding: "utf8",
  });
  const folder = join(common.trim(), "metrics", "running");

  mkdirSync(folder, { recursive: true });

  const call = runningCall(input, Temporal.Now.instant().epochMilliseconds);

  writeFileSync(join(folder, `${id}.json`), JSON.stringify(call));
}

// The lanes of `check:all` and the command of each check, as `check-all --list` prints them: the
// commands live in `tools/repo-tools/src/check_all.rs`, not in `package.json` (ADR-0103).
import { spawnSync } from "node:child_process";
import { join } from "node:path";

/** One check of a lane: its lane's name, its own name and the command it runs. */
type ListedCheck = { readonly lane: string; readonly name: string; readonly command: string };

/**
 * Every check of every named lane, in the lanes' order: the release binary built when it is not
 * yet, which `check:all` does before its lanes and `npm ci`'s `prepare` in the CI.
 */
export const CHECK_ALL_LANES: readonly ListedCheck[] = spawnSync(
  "cargo",
  ["run", "--release", "-q", "-p", "repo-tools", "--bin", "check-all", "--", "--list"],
  { cwd: join(import.meta.dirname, ".."), encoding: "utf8" },
)
  .stdout.split("\n")
  .filter((line) => line !== "")
  .map((line) => {
    const [lane = "", name = "", ...command] = line.split("\t");

    return { command: command.join("\t"), lane, name };
  });

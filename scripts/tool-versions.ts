// Latest versions of the tools outside npm (docs/tooling/versions-and-security.md): every GitHub
// Action of the workflows on its latest major version, and Node.js on its latest LTS major.
// Run by `npm run deps:tools` in `check:all`; needs the network and, for GitHub, a token
// (GITHUB_TOKEN in CI, `gh auth token` locally) to stay under the API rate limit.

import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

/** An action used by a workflow, with the major version it is pinned to. */
type ActionUse = {
  readonly major: number;
  readonly repository: string;
};

/** Folder of the GitHub workflows. */
const WORKFLOWS = ".github/workflows";

/** Official index of the Node.js releases, newest first. */
const NODE_RELEASES = "https://nodejs.org/dist/index.json";

/**
 * Property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property, or `undefined` when `value` is not an object
 */
function field(value: unknown, key: string): unknown {
  return typeof value === "object" && value !== null ? Reflect.get(value, key) : undefined;
}

/**
 * Actions used by a workflow, pinned as `owner/repository[/path]@vN`.
 *
 * @param workflow - text of a workflow file
 * @returns each `uses:` line, its repository and its major version
 */
export function actionUses(workflow: string): readonly ActionUse[] {
  return workflow
    .matchAll(/uses:\s*(?<repository>[\w.-]+\/[\w.-]+)(?:\/[\w./-]+)?@v(?<major>\d+)\b/gu)
    .map((match) => ({
      major: Number(match.groups?.["major"]),
      repository: match.groups?.["repository"] ?? "",
    }))
    .toArray();
}

/**
 * Whether a list of Git tag names holds the given major version (`vN`, `vN.x`, `vN.x.y`).
 *
 * @param tags - tag names, without `refs/tags/`
 * @param major - major version looked for
 * @returns true when a tag of that major exists
 */
export function hasMajor(tags: readonly string[], major: number): boolean {
  const pattern = new RegExp(String.raw`^v${String(major)}(?:\.|$)`, "u");

  return tags.some((tag) => pattern.test(tag));
}

/**
 * Major version of the latest Node.js LTS release, from the official release index.
 *
 * @param releases - parsed JSON of the release index, newest first
 * @returns the major version of the first entry whose `lts` is a codename, or NaN
 */
export function latestLtsMajor(releases: unknown): number {
  const entries: readonly unknown[] = Array.isArray(releases) ? releases : [];
  const lts = entries.find((release) => typeof field(release, "lts") === "string");
  const version = field(lts, "version");

  return Number(
    /^v(?<major>\d+)/u.exec(typeof version === "string" ? version : "")?.groups?.["major"],
  );
}

/**
 * GitHub token: from the environment in CI, from the GitHub CLI locally.
 *
 * @returns a token, or an empty string when none is available
 */
function gitHubToken(): string {
  const fromEnvironment = process.env["GITHUB_TOKEN"] ?? process.env["GH_TOKEN"];

  if (fromEnvironment !== undefined) {
    return fromEnvironment;
  }

  try {
    return execFileSync("gh", ["auth", "token"], { encoding: "utf8" }).trim();
  } catch {
    return "";
  }
}

/**
 * What is wrong with one action: a newer major version exists, or it could not be checked.
 *
 * @param use - action and its pinned major version
 * @param token - GitHub token, possibly empty
 * @returns a line for the report, or an empty string when the action is up-to-date
 */
async function actionProblem(use: ActionUse, token: string): Promise<string> {
  const next = use.major + 1;
  const url = `https://api.github.com/repos/${use.repository}/git/matching-refs/tags/v${String(next)}`;
  const response = await fetch(
    url,
    token === "" ? {} : { headers: { authorization: `Bearer ${token}` } },
  );

  if (!response.ok) {
    return `${use.repository}: not checked (HTTP ${String(response.status)})`;
  }

  const references: unknown = await response.json();
  const tags = (Array.isArray(references) ? references : []).map((entry) =>
    String(field(entry, "ref")).replace("refs/tags/", ""),
  );

  return hasMajor(tags, next)
    ? `${use.repository}@v${String(use.major)} → v${String(next)} or later`
    : "";
}

/**
 * Report lines for the actions of every workflow.
 *
 * @returns one line per outdated or unchecked action
 */
async function actionProblems(): Promise<readonly string[]> {
  const token = gitHubToken();
  const uses = readdirSync(WORKFLOWS)
    .filter((name) => name.endsWith(".yml"))
    .flatMap((name) => actionUses(readFileSync(join(WORKFLOWS, name), "utf8")));
  const unique = new Map(uses.map((use) => [`${use.repository}@${String(use.major)}`, use]));
  const lines = await Promise.all(
    unique.values().map(async (use) => await actionProblem(use, token)),
  );

  return lines.filter((line) => line !== "");
}

/**
 * Report line for Node.js, when `.nvmrc` is not on the latest LTS major.
 *
 * @returns zero or one line
 */
async function nodeProblems(): Promise<readonly string[]> {
  const response = await fetch(NODE_RELEASES);
  const lts = latestLtsMajor(response.ok ? await response.json() : []);
  const pinned = Number(readFileSync(".nvmrc", "utf8").trim());

  return pinned === lts ? [] : [`Node.js ${String(pinned)} (.nvmrc) → ${String(lts)} (latest LTS)`];
}

if (import.meta.main) {
  const problems = [...(await actionProblems()), ...(await nodeProblems())];

  if (problems.length > 0) {
    process.stderr.write(
      `Tools not up-to-date:\n${problems.map((line) => `- ${line}`).join("\n")}\n`,
    );

    process.exitCode = 1;
  } else {
    process.stdout.write("GitHub Actions and Node.js (LTS) up-to-date\n");
  }
}

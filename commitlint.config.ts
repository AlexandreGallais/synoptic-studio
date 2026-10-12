import { RuleConfigSeverity } from "@commitlint/types";

import type { UserConfig } from "@commitlint/types";

/** Maximum length of the commit header and body lines (same as `max_line_length`). */
const MAX_LINE_LENGTH = 100;

/** Allowed commit scopes: one per layer, plus project-wide areas. */
const SCOPES = [
  "adr",
  "backlog",
  "ci",
  "deps",
  "docs",
  "eslint",
  "geometry",
  "interaction",
  "io",
  "math",
  "model",
  "playground",
  "render",
  "routing",
  "tooling",
];

/**
 * Whether a commit was written by release-please, whose release commits have a fixed format.
 *
 * @param message - full commit message
 * @returns `true` for a release commit
 */
function isReleaseCommit(message: string): boolean {
  return message.startsWith("chore(main): release");
}

/**
 * Whether a commit was written by Dependabot, whose bodies hold long links.
 *
 * @param message - full commit message
 * @returns `true` for a Dependabot commit
 */
function isDependabotCommit(message: string): boolean {
  return message.includes("Signed-off-by: dependabot[bot]");
}

/**
 * Commit message policy: Conventional Commits (`type(scope): subject`).
 *
 * @see docs/tooling/git-workflow.md
 */
const CONFIG: UserConfig = {
  extends: ["@commitlint/config-conventional"],
  ignores: [isReleaseCommit, isDependabotCommit],
  rules: {
    "body-max-line-length": [RuleConfigSeverity.Error, "always", MAX_LINE_LENGTH],
    "header-max-length": [RuleConfigSeverity.Error, "always", MAX_LINE_LENGTH],
    "scope-enum": [RuleConfigSeverity.Error, "always", SCOPES],
  },
};

export default CONFIG;

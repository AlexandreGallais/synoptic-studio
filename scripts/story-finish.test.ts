import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  cleanUp,
  cleanUpPlan,
  nextStep,
  rebaseOnBase,
  rowsMessage,
  worktrees,
} from "./story-finish";

/** The feature branch the stories of a test merge into. */
const BASE = "feature/f99-topic";

/** The story finished in a test. */
const STORY = "chore/en-999-topic";

/** Folder of this file's repositories, removed at the end. */
const TEMPORARY = mkdtempSync(join(tmpdir(), "story-finish-"));

/** The same folder, its links resolved as Git prints it. */
const SCRATCH = realpathSync(TEMPORARY);

/** The environment before the tests, given back at the end. */
const SAVED = { ...process.env };

/**
 * Output of Git in a folder.
 *
 * @param cwd - the folder
 * @param options - Git's arguments
 * @returns its trimmed standard output
 */
function git(cwd: string, ...options: readonly string[]): string {
  return execFileSync("git", options, { cwd, encoding: "utf8", stdio: "pipe" }).trim();
}

/**
 * Writes a file and commits it.
 *
 * @param cwd - the checkout
 * @param file - the file's name
 * @param text - its content
 */
function commit(cwd: string, file: string, text: string): void {
  writeFileSync(join(cwd, file), text);
  git(cwd, "add", file);
  git(cwd, "commit", "--quiet", "-m", `write ${file}`);
}

/**
 * A remote and two clones: `alpha` on the story's branch, `beta` on the base.
 *
 * @param name - the folder of this test's repositories
 * @returns the paths of the two clones
 */
function repositories(name: string): { alpha: string; beta: string } {
  const folder = join(SCRATCH, name);
  const origin = join(folder, "origin.git");
  const alpha = join(folder, "alpha");
  const beta = join(folder, "beta");

  mkdirSync(folder);
  git(folder, "init", "--quiet", "--bare", origin);
  git(folder, "clone", "--quiet", origin, alpha);
  commit(alpha, "readme", "start\n");
  git(alpha, "push", "--quiet", "-u", "origin", `HEAD:${BASE}`);
  git(alpha, "switch", "--quiet", "-c", STORY);
  commit(alpha, "story", "story\n");
  git(alpha, "push", "--quiet", "-u", "origin", STORY);
  git(folder, "clone", "--quiet", "--branch", BASE, origin, beta);

  return { alpha, beta };
}

/**
 * A remote and its main checkout on the base, the story in a linked worktree, as in a run.
 *
 * @param name - the folder of this test's repositories
 * @returns the paths of the main checkout and of the story's worktree
 */
function runCheckouts(name: string): { main: string; worktree: string } {
  const folder = join(SCRATCH, name);
  const origin = join(folder, "origin.git");
  const main = join(folder, "main");
  const worktree = join(folder, "en-999");

  mkdirSync(folder);
  git(folder, "init", "--quiet", "--bare", origin);
  git(folder, "clone", "--quiet", origin, main);
  commit(main, "readme", "start\n");
  git(main, "switch", "--quiet", "-c", BASE);
  git(main, "push", "--quiet", "-u", "origin", BASE);
  git(main, "worktree", "add", "--quiet", "-b", STORY, worktree, BASE);
  commit(worktree, "story", "story\n");
  git(worktree, "push", "--quiet", "-u", "origin", STORY);

  return { main, worktree };
}

beforeAll(() => {
  // The pre-push hook sets GIT_DIR: inherited, it would point these repositories' commands at the
  // real one (EN-050).
  for (const key of Object.keys(process.env)) {
    if (key.startsWith("GIT_")) {
      Reflect.deleteProperty(process.env, key);
    }
  }

  const config = join(SCRATCH, "gitconfig");

  writeFileSync(
    config,
    "[user]\n\tname = Test\n\temail = test@example.com\n[commit]\n\tgpgsign = false\n[init]\n\tdefaultBranch = main\n",
  );

  process.env["GIT_CONFIG_GLOBAL"] = config;
  process.env["GIT_CONFIG_NOSYSTEM"] = "1";
});

afterAll(() => {
  process.env = SAVED;
  rmSync(SCRATCH, { force: true, recursive: true });
});

describe("story-finish", () => {
  it("cleans once merged, waits while open for three minutes, then stops", () => {
    expect(nextStep("MERGED", 1)).toBe("clean");
    expect(nextStep("OPEN", 17)).toBe("wait");
    expect(nextStep("OPEN", 18)).toBe("stop");
    expect(nextStep("CLOSED", 1)).toBe("stop");
  });

  it("names the merged story in the commit moving its rows into the logs", () => {
    expect(rowsMessage("chore/en-050-shared-files-written-at-merge")).toBe(
      "docs(docs): move the rows of EN-050 into the logs\n\nRefs: EN-050\n",
    );

    expect(rowsMessage("feat/us-061-font-list")).toBe(
      "docs(docs): move the rows of US-061 into the logs\n\nRefs: US-061\n",
    );
  });

  it("[F18.AC7] rebases the next story on its base once another story merged into it", () => {
    const { alpha, beta } = repositories("rebased");

    commit(beta, "other", "another story\n");
    git(beta, "push", "--quiet");

    expect(rebaseOnBase(BASE, alpha)).toBe("rebased");
    expect(git(alpha, "log", "--format=%s", "-3")).toBe("write story\nwrite other\nwrite readme");
    expect(git(alpha, "branch", "--show-current")).toBe(STORY);
  });

  it("leaves a story already on its base as it is", () => {
    const { alpha } = repositories("up-to-date");
    const head = git(alpha, "rev-parse", "HEAD");

    expect(rebaseOnBase(BASE, alpha)).toBe("up to date");
    expect(git(alpha, "rev-parse", "HEAD")).toBe(head);
  });

  it("aborts a rebase that meets a conflict and leaves the branch as it was", () => {
    const { alpha, beta } = repositories("conflict");
    const head = git(alpha, "rev-parse", "HEAD");

    commit(beta, "story", "the same file, written otherwise\n");
    git(beta, "push", "--quiet");

    expect(() => rebaseOnBase(BASE, alpha)).toThrow(`conflict rebasing on origin/${BASE}`);
    expect(git(alpha, "rev-parse", "HEAD")).toBe(head);
    expect(git(alpha, "status", "--porcelain")).toBe("");
    expect(git(alpha, "branch", "--show-current")).toBe(STORY);
  });

  it("reads each checkout's folder and branch, the main one first", () => {
    const porcelain = [
      "worktree /repo\nHEAD 1111\nbranch refs/heads/feature/f18-topic\n",
      "worktree /repo/.claude/worktrees/en-051\nHEAD 2222\nbranch refs/heads/chore/en-051-topic\n",
      "worktree /repo/.claude/worktrees/spare\nHEAD 3333\ndetached\n",
    ].join("\n");

    expect(worktrees(porcelain)).toStrictEqual([
      { branch: "feature/f18-topic", path: "/repo" },
      { branch: "chore/en-051-topic", path: "/repo/.claude/worktrees/en-051" },
      { branch: "", path: "/repo/.claude/worktrees/spare" },
    ]);

    expect(worktrees("")).toStrictEqual([]);
  });

  it("from a worktree, updates the base where it is checked out, then removes the worktree", () => {
    const here = "/repo/.claude/worktrees/en-999";

    expect(
      cleanUpPlan({ base: BASE, branch: STORY }, here, [
        { branch: BASE, path: "/repo" },
        { branch: STORY, path: here },
      ]),
    ).toStrictEqual({
      place: "/repo",
      remove: [
        { cwd: "/repo", options: ["worktree", "remove", here] },
        { cwd: "/repo", options: ["branch", "-D", STORY] },
      ],
      update: [
        { cwd: here, options: ["fetch", "--quiet", "--prune", "origin"] },
        { cwd: "/repo", options: ["merge", "--ff-only", "--quiet", `origin/${BASE}`] },
      ],
    });
  });

  it("from a worktree whose base no checkout holds, switches the worktree to the base first", () => {
    const here = "/repo/.claude/worktrees/en-999";
    const plan = cleanUpPlan({ base: BASE, branch: STORY }, here, [
      { branch: "develop", path: "/repo" },
      { branch: STORY, path: here },
    ]);

    expect(plan.place).toBe(here);

    expect(plan.update.map((step) => [step.cwd, step.options[0]])).toStrictEqual([
      [here, "fetch"],
      [here, "switch"],
      [here, "merge"],
    ]);

    expect(plan.remove.map((step) => step.options[0])).toStrictEqual(["worktree", "branch"]);
  });

  it("from the main checkout, switches to the base and removes no worktree", () => {
    const plan = cleanUpPlan({ base: BASE, branch: STORY }, "/repo", [
      { branch: STORY, path: "/repo" },
      { branch: BASE, path: "/repo/.claude/worktrees/other" },
    ]);

    expect(plan.place).toBe("/repo");
    expect(plan.update.map((step) => step.options[0])).toStrictEqual(["fetch", "switch", "merge"]);
    expect(plan.remove).toStrictEqual([{ cwd: "/repo", options: ["branch", "-D", STORY] }]);
  });

  it("[F18.AC7] cleans up a merged story from its worktree without switching the main checkout", () => {
    const { main, worktree } = runCheckouts("merged");
    const places: string[] = [];

    // GitHub's merge of the pull request, as seen from the remote.
    git(worktree, "push", "--quiet", "origin", `${STORY}:${BASE}`);

    const plan = cleanUpPlan(
      { base: BASE, branch: STORY },
      worktree,
      worktrees(git(worktree, "worktree", "list", "--porcelain")),
    );

    expect(
      cleanUp("MERGED", plan, (cwd) => {
        places.push(cwd);
      }),
    ).toBe(true);

    expect(places).toStrictEqual([main]);
    expect(existsSync(worktree)).toBe(false);
    expect(git(main, "branch", "--list", STORY)).toBe("");
    expect(git(main, "branch", "--show-current")).toBe(BASE);
    expect(git(main, "rev-parse", "HEAD")).toBe(git(main, "rev-parse", `origin/${BASE}`));
    expect(git(main, "log", "--format=%s", "-1")).toBe("write story");
  });

  it("[F18.AC7] keeps the worktree and the branch of a pull request not merged", () => {
    const { main, worktree } = runCheckouts("open");
    const places: string[] = [];
    const plan = cleanUpPlan(
      { base: BASE, branch: STORY },
      worktree,
      worktrees(git(worktree, "worktree", "list", "--porcelain")),
    );

    for (const state of ["OPEN", "CLOSED"]) {
      expect(
        cleanUp(state, plan, (cwd) => {
          places.push(cwd);
        }),
      ).toBe(false);
    }

    expect(places).toStrictEqual([]);
    expect(existsSync(worktree)).toBe(true);
    expect(git(main, "branch", "--list", STORY)).toBe(`+ ${STORY}`);
    expect(git(worktree, "branch", "--show-current")).toBe(STORY);
  });
});

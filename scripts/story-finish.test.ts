import { execFileSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  realpathSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { delimiter, join } from "node:path";

import { afterAll, beforeAll, describe, expect, it } from "vitest";

import { nextStep, rebaseOnBase, startStep } from "./story-finish";
import { cleanUp, cleanUpPlan, manualSteps, worktrees } from "./story-finish-plan";
import { commitRows, pushAhead, rowsMessage } from "./story-finish-rows";

/** The feature branch the stories of a test merge into. */
const BASE = "feature/f99-topic";

/** The story finished in a test. */
const STORY = "chore/en-999-topic";

/** The repository on GitHub. */
const REPOSITORY = "owner/name";

/** The story, its base and its repository, as `story:finish` reads them. */
const STORY_OF_BASE = { base: BASE, branch: STORY, repository: REPOSITORY };

/** The call deleting the story's branch on GitHub. */
const REMOTE_DELETION = ["api", "-X", "DELETE", `repos/${REPOSITORY}/git/refs/heads/${STORY}`];

/** Folder of this file's repositories, removed at the end. */
const TEMPORARY = mkdtempSync(join(tmpdir(), "story-finish-"));

/** The same folder, its links resolved as Git prints it. */
const SCRATCH = realpathSync(TEMPORARY);

/** The environment before the tests, given back at the end. */
const SAVED = { ...process.env };

/** Where the stub of GitHub's command line writes each call: its folder, then its arguments. */
const GH_CALLS = join(SCRATCH, "gh-calls");

/**
 * The calls made to the stub of GitHub's command line since the last reading, which empties them.
 *
 * @returns each call's folder and arguments, one line each
 */
function ghCalls(): readonly string[] {
  const calls = existsSync(GH_CALLS) ? readFileSync(GH_CALLS, "utf8").trim().split("\n") : [];

  rmSync(GH_CALLS, { force: true });

  return calls;
}

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

  // GitHub's command line replaced by a stub first on PATH: it writes where it runs and what for.
  const bin = join(SCRATCH, "bin");

  mkdirSync(bin);
  writeFileSync(join(bin, "gh"), `#!/bin/sh\necho "$PWD $*" >> "${GH_CALLS}"\n`, { mode: 0o755 });
  process.env["PATH"] = `${bin}${delimiter}${process.env["PATH"] ?? ""}`;
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

  it("opens a pull request when there is none, awaits an open one, cleans up after a merged one", () => {
    expect(startStep("")).toBe("create");
    expect(startStep("CLOSED")).toBe("create");
    expect(startStep("OPEN")).toBe("await");
    expect(startStep("MERGED")).toBe("clean");
  });

  it("commits only the files the rows went into, whatever else is staged in the base", () => {
    const { main } = runCheckouts("rows-only");

    commit(main, "journal.md", "# Journal\n");
    writeFileSync(join(main, "journal.md"), "# Journal\n\n- `#run` a row\n");
    // The main thread's own work, staged in the same checkout.
    writeFileSync(join(main, "draft.md"), "a draft\n");
    git(main, "add", "draft.md");

    commitRows(STORY, ["journal.md"], main);

    expect(git(main, "show", "--name-only", "--format=%s", "HEAD")).toBe(
      "docs(docs): move the rows of EN-999 into the logs\n\njournal.md",
    );

    expect(git(main, "status", "--porcelain")).toBe("A  draft.md");
  });

  it("pushes the base when it holds commits its remote has not, and only then", () => {
    const { main } = runCheckouts("ahead");

    expect(pushAhead(main)).toBe(false);

    // The rows' commit of a run stopped before its push.
    commit(main, "journal.md", "# Journal\n");

    expect(pushAhead(main)).toBe(true);
    expect(git(main, "rev-parse", `origin/${BASE}`)).toBe(git(main, "rev-parse", "HEAD"));
    expect(pushAhead(main)).toBe(false);
  });

  it("writes the clean-up's steps to type when it stopped midway, in their order", () => {
    const here = "/repo/.claude/worktrees/en-999";
    const plan = cleanUpPlan(STORY_OF_BASE, here, [
      { branch: BASE, path: "/repo" },
      { branch: STORY, path: here },
    ]);

    expect(manualSteps(plan, STORY).split("\n")).toStrictEqual([
      `cd "${here}" && git fetch --quiet --prune origin`,
      `cd "/repo" && git merge --ff-only --quiet origin/${BASE}`,
      `cd "/repo" && scripts/rust-tool.sh move-rows ${STORY}`,
      `cd "/repo" && git commit -m "docs(docs): move the rows of EN-999 into the logs" -m "Refs: EN-999" -- <the files it printed>`,
      `cd "/repo" && git push`,
      `cd "/repo" && git worktree remove ${here}`,
      `cd "/repo" && git branch -D ${STORY}`,
      `cd "/repo" && gh ${REMOTE_DELETION.join(" ")}`,
    ]);
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
      cleanUpPlan(STORY_OF_BASE, here, [
        { branch: BASE, path: "/repo" },
        { branch: STORY, path: here },
      ]),
    ).toStrictEqual({
      place: "/repo",
      remove: [
        { command: "git", cwd: "/repo", options: ["worktree", "remove", here] },
        { command: "git", cwd: "/repo", options: ["branch", "-D", STORY] },
        { command: "gh", cwd: "/repo", options: REMOTE_DELETION },
      ],
      update: [
        { command: "git", cwd: here, options: ["fetch", "--quiet", "--prune", "origin"] },
        {
          command: "git",
          cwd: "/repo",
          options: ["merge", "--ff-only", "--quiet", `origin/${BASE}`],
        },
      ],
    });
  });

  it("from a worktree whose base no checkout holds, switches the worktree to the base first", () => {
    const here = "/repo/.claude/worktrees/en-999";
    const plan = cleanUpPlan(STORY_OF_BASE, here, [
      { branch: "develop", path: "/repo" },
      { branch: STORY, path: here },
    ]);

    expect(plan.place).toBe(here);

    expect(plan.update.map((step) => [step.cwd, step.options[0]])).toStrictEqual([
      [here, "fetch"],
      [here, "switch"],
      [here, "merge"],
    ]);

    expect(plan.remove.map((step) => [step.cwd, step.options[0]])).toStrictEqual([
      ["/repo", "worktree"],
      ["/repo", "branch"],
      ["/repo", "api"],
    ]);
  });

  it("from the main checkout, switches to the base and removes no worktree", () => {
    const plan = cleanUpPlan(STORY_OF_BASE, "/repo", [
      { branch: STORY, path: "/repo" },
      { branch: BASE, path: "/repo/.claude/worktrees/other" },
    ]);

    expect(plan.place).toBe("/repo");
    expect(plan.update.map((step) => step.options[0])).toStrictEqual(["fetch", "switch", "merge"]);

    expect(plan.remove).toStrictEqual([
      { command: "git", cwd: "/repo", options: ["branch", "-D", STORY] },
      { command: "gh", cwd: "/repo", options: REMOTE_DELETION },
    ]);
  });

  it("[F18.AC7] cleans up a merged story from its worktree without switching the main checkout", () => {
    const { main, worktree } = runCheckouts("merged");
    const places: string[] = [];

    // GitHub's merge of the pull request, as seen from the remote.
    git(worktree, "push", "--quiet", "origin", `${STORY}:${BASE}`);

    const plan = cleanUpPlan(
      STORY_OF_BASE,
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
    expect(ghCalls()).toStrictEqual([`${main} ${REMOTE_DELETION.join(" ")}`]);
  });

  it("[F18.AC7] deletes the branch on GitHub from the main checkout once the worktree is removed", () => {
    const { main, worktree } = runCheckouts("base-in-no-checkout");
    const places: string[] = [];

    // As in a run: the main checkout on another branch, the base checked out nowhere (PR #272).
    git(main, "switch", "--quiet", "main");
    git(worktree, "push", "--quiet", "origin", `${STORY}:${BASE}`);

    const plan = cleanUpPlan(
      STORY_OF_BASE,
      worktree,
      worktrees(git(worktree, "worktree", "list", "--porcelain")),
    );

    expect(
      cleanUp("MERGED", plan, (cwd) => {
        places.push(cwd);
      }),
    ).toBe(true);

    expect(places).toStrictEqual([worktree]);
    expect(existsSync(worktree)).toBe(false);
    expect(git(main, "branch", "--show-current")).toBe("main");
    expect(git(main, "branch", "--list", STORY)).toBe("");
    expect(git(main, "rev-parse", BASE)).toBe(git(main, "rev-parse", `origin/${BASE}`));
    expect(ghCalls()).toStrictEqual([`${main} ${REMOTE_DELETION.join(" ")}`]);
  });

  it("[F18.AC7] keeps the worktree and the branch of a pull request not merged", () => {
    const { main, worktree } = runCheckouts("open");
    const places: string[] = [];
    const plan = cleanUpPlan(
      STORY_OF_BASE,
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
    expect(ghCalls()).toStrictEqual([]);
    expect(existsSync(worktree)).toBe(true);
    expect(git(main, "branch", "--list", STORY)).toBe(`+ ${STORY}`);
    expect(git(worktree, "branch", "--show-current")).toBe(STORY);
  });
});

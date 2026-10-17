import { describe, expect, it } from "vitest";

import { commandOf, deniedReason, worktreeReason } from "./guard-bash";

/** A story's worktree, as a worker's Bash calls see it. */
const WORKTREE = "/work/SVG editor/.claude/worktrees/us-061";

describe("guard-bash hook", () => {
  it.each([
    "gh pr merge 21 --rebase",
    "cd repo && gh pr merge 21",
    "git status; gh pr merge 21",
    "git status\ngh pr merge 21",
    "false || gh pr merge 21",
    'git commit --no-verify -m "x"',
    "git push --no-verify origin main",
    "git push --force origin feat/x",
    "git push -f",
    "git push origin +feat/x",
    'git -C "/work/SVG editor" push --force',
    "git -c user.name=x commit --no-verify",
    'git commit -nm "x"',
    'git commit -n -m "x"',
    'HUSKY=0 git commit -m "x"',
    "export HUSKY=0",
    "git config core.hooksPath /dev/null",
    "git -c core.hooksPath=/dev/null commit -m x",
    "CI=1 HUSKY=0 git push",
    "cat > a.txt <<'EOF'\ntext\nEOF\ngh pr merge 21",
    "npx vitest run",
    "npx vitest run --coverage",
    "cd repo && npx vitest run --reporter=dot",
    "npx vitest",
    // Every test run all the same (AUD-020): the current folder, a project, the binary itself.
    "npx vitest run .",
    "npx vitest run ./ --coverage",
    "npx vitest run --project unit",
    "npx vitest --reporter dot run",
    "node_modules/.bin/vitest run",
    "./node_modules/.bin/vitest run --project x",
  ])("denies %s", (command) => {
    expect(deniedReason(command)).toBeDefined();
  });

  it.each([
    "gh pr view 21",
    "gh pr create --fill",
    "git push --force-with-lease origin feat/x",
    "git push origin main",
    'git -C "/work/SVG editor" push -u origin feat/x',
    'git commit -m "x" --amend',
    "git log -n 3",
    "sed -i '' 's/a/denies HUSKY=0 and core.hooksPath/' docs/tooling/agent.md",
    'git commit -m "docs: explain why gh pr merge is denied"',
    "npm run check:all",
    "npx vitest run scripts/check-all.test.ts",
    "npx vitest run src/geometry --coverage",
    "npx vitest run --project unit scripts/story-finish.test.ts",
    "node_modules/.bin/vitest run .claude/hooks/guard-bash.test.ts",
    "npx vitest-preview",
    "npm run test",
    "cat > note.md <<'EOF'\nThe agent never runs gh pr merge.\nEOF",
    'git commit -F - <<"MSG"\ndocs: x\n\ngit push --force is denied\nMSG\ngit log -1',
  ])("allows %s", (command) => {
    expect(deniedReason(command)).toBeUndefined();
  });

  it.each([
    `cd "${WORKTREE}" && git status`,
    "git add -A && git commit -m x",
    "git fetch origin; git rebase origin/develop",
    "git log --oneline | head -5",
    "npm run fix && git add scripts",
  ])("[F18.AC8] denies %s in a worktree, with the form that works", (command) => {
    const reason = worktreeReason(command, WORKTREE) ?? "";

    // A subagent's calls start in the main checkout: the hint enters the worktree first.
    expect(reason).toMatch(
      /^A worktree refuses a chained Git command: run `cd "\/work\/SVG editor\/\.claude\/worktrees\/us-061" \|\| exit 9` then `git [^`]+` alone\.(?: Or in one call: .+\.)?$/u,
    );

    expect(reason).not.toContain("\n");
    // From a folder inside the worktree, the hint still names the worktree's root.
    expect(worktreeReason(command, `${WORKTREE}/scripts`)).toBe(reason);
  });

  it.each([
    "git status",
    "git commit -F - <<'EOF'\nfix: x\n\ngit push && git log\nEOF",
    "npm run fix && npx tsc --noEmit",
    "ls docs; cat AGENTS.md",
  ])("allows %s in a worktree", (command) => {
    expect(worktreeReason(command, WORKTREE)).toBeUndefined();
    expect(worktreeReason(command, `${WORKTREE}/scripts`)).toBeUndefined();
  });

  it("names the Rust command of a frequent Git sequence (EN-100)", () => {
    expect(worktreeReason("git add -A && git commit -m x", WORKTREE)).toMatch(
      /Or in one call: `npm run git:commit -- -F <message file> \[--push\] <paths>`[^;]*\.$/u,
    );

    expect(worktreeReason("git fetch origin; git rebase origin/develop", WORKTREE)).toMatch(
      /Or in one call: `npm run git:sync` fetches and rebases [^;]*\.$/u,
    );

    expect(worktreeReason("git log --oneline | head -5", WORKTREE)).not.toContain("one call");
  });

  it.each([
    `cd "${WORKTREE}" || exit 9; npm run git:sync`,
    `cd "${WORKTREE}" || exit 9\nnpm run git:commit -- -F - --push a.md <<'EOF'\nchore(tooling): x\n\nRefs: EN-100.T1\nEOF`,
  ])("allows the Rust command of a Git sequence in a worktree: %s", (command) => {
    expect(deniedReason(command)).toBeUndefined();
    expect(worktreeReason(command, WORKTREE)).toBeUndefined();
  });

  it("allows a chained Git command outside a worktree", () => {
    expect(worktreeReason("git add -A && git commit -m x", "/work/SVG editor")).toBeUndefined();
    expect(worktreeReason("git status && git log", "/work/.claude/worktrees-old")).toBeUndefined();
  });

  it("[F18.AC8] refuses golden:write in a worktree and says why", () => {
    expect(worktreeReason("npm run golden:write", WORKTREE)).toMatch(/shared by every worktree/u);
    expect(worktreeReason("npm run golden:write", "/work/SVG editor")).toBeUndefined();
    expect(worktreeReason("npm run golden:check", WORKTREE)).toBeUndefined();
  });

  it("reads the command of a Bash hook input, and nothing from anything else", () => {
    // Hook inputs are external JSON (snake_case keys): parsed as the hook parses them.
    expect(commandOf(JSON.parse('{"tool_input":{"command":"ls"},"tool_name":"Bash"}'))).toBe("ls");
    expect(commandOf(JSON.parse('{"tool_input":{"file_path":"a.ts"}}'))).toBe("");
    expect(commandOf(JSON.parse("null"))).toBe("");
  });
});

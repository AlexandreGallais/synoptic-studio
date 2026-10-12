import { describe, expect, it } from "vitest";

import { commandOf, deniedReason } from "./guard-bash";

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
    "cat > a.txt <<'EOF'\ntext\nEOF\ngh pr merge 21",
  ])("denies %s", (command) => {
    expect(deniedReason(command)).toBeDefined();
  });

  it.each([
    "gh pr view 21",
    "gh pr create --fill",
    "git push --force-with-lease origin feat/x",
    "git push origin main",
    'git commit -m "docs: explain why gh pr merge is denied"',
    "npm run check:all",
    "cat > note.md <<'EOF'\nThe agent never runs gh pr merge.\nEOF",
    'git commit -F - <<"MSG"\ndocs: x\n\ngit push --force is denied\nMSG\ngit log -1',
  ])("allows %s", (command) => {
    expect(deniedReason(command)).toBeUndefined();
  });

  it("reads the command of a Bash hook input, and nothing from anything else", () => {
    // Hook inputs are external JSON (snake_case keys): parsed as the hook parses them.
    expect(commandOf(JSON.parse('{"tool_input":{"command":"ls"},"tool_name":"Bash"}'))).toBe("ls");
    expect(commandOf(JSON.parse('{"tool_input":{"file_path":"a.ts"}}'))).toBe("");
    expect(commandOf(JSON.parse("null"))).toBe("");
  });
});

import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { RuleTester } from "@typescript-eslint/rule-tester";

import { FOLDER_HAS_INDEX_RULE } from "./folder-has-index";

/** Fixture tree: one folder with a barrel, one without. */
const ROOT = mkdtempSync(join(tmpdir(), "folder-index-"));

mkdirSync(join(ROOT, "with"));
mkdirSync(join(ROOT, "without"));
writeFileSync(join(ROOT, "with/index.ts"), "export {};\n");

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Rule options shared by the cases. */
const OPTIONS = [{ exemptDirectories: ["."], root: ROOT }] as const;

TESTER.run("folder-has-index", FOLDER_HAS_INDEX_RULE, {
  invalid: [
    {
      code: "export {};",
      errors: [{ messageId: "missing" }],
      filename: join(ROOT, "without/a.ts"),
      options: OPTIONS,
    },
  ],
  valid: [{ code: "export {};", filename: join(ROOT, "with/a.ts"), options: OPTIONS }],
});

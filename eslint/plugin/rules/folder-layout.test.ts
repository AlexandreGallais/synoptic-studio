import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { RuleTester } from "@typescript-eslint/rule-tester";

import { FOLDER_LAYOUT_RULE } from "./folder-layout";

/** Fixture tree: a folder of domains, a mixed one, a full one, a listed one. */
const ROOT = mkdtempSync(join(tmpdir(), "folder-layout-"));

for (const folder of ["domains/a", "mixed/a", "full", "listed"]) {
  mkdirSync(join(ROOT, folder), { recursive: true });
}

for (const folder of ["full", "listed"]) {
  for (const index of [1, 2, 3]) {
    writeFileSync(join(ROOT, folder, `m${String(index)}.ts`), "export {};\n");
  }

  writeFileSync(join(ROOT, folder, "m1.test.ts"), "export {};\n");
}

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Rule options shared by the cases: a signal of 2, one folder listed. */
const OPTIONS = [{ listed: ["listed"], root: ROOT, signal: 2 }] as const;
/** Code of every case: the rule reads only the file's place. */
const CODE = "export {};";

TESTER.run("folder-layout", FOLDER_LAYOUT_RULE, {
  invalid: [
    {
      code: CODE,
      errors: [{ messageId: "mixed" }],
      filename: join(ROOT, "mixed/b.ts"),
      options: OPTIONS,
    },
    {
      code: CODE,
      errors: [{ messageId: "mixed" }],
      filename: join(ROOT, "mixed/b.test.ts"),
      options: OPTIONS,
    },
    {
      code: CODE,
      errors: [{ data: { modules: 3, path: "full", signal: 2 }, messageId: "past" }],
      filename: join(ROOT, "full/m1.ts"),
      options: OPTIONS,
    },
  ],
  valid: [
    { code: CODE, filename: join(ROOT, "domains/index.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "domains/a/x.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "full/m1.test.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "full/index.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "listed/m1.ts"), options: OPTIONS },
  ],
});

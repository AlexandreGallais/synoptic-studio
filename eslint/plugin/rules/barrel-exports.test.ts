import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { RuleTester } from "@typescript-eslint/rule-tester";

import { BARREL_EXPORTS_RULE } from "./barrel-exports";

/** Fixture folder: two modules, a test, a sub-folder with BARREL, a sub-folder without. */
const ROOT = mkdtempSync(join(tmpdir(), "barrel-"));

mkdirSync(join(ROOT, "arc"));
mkdirSync(join(ROOT, "assets"));

for (const file of [
  "index.ts",
  "vector.ts",
  "angle.ts",
  "angle.test.ts",
  "arc/index.ts",
  "assets/logo.svg",
]) {
  writeFileSync(join(ROOT, file), "export {};\n");
}

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Barrel file under test. */
const BARREL = join(ROOT, "index.ts");
/** Barrel content matching the fixture folder. */
const EXPECTED = 'export * from "./angle";\nexport * from "./arc";\nexport * from "./vector";\n';

TESTER.run("barrel-exports", BARREL_EXPORTS_RULE, {
  invalid: [
    {
      code: 'export * from "./vector";',
      errors: [{ messageId: "outOfSync" }],
      filename: BARREL,
      output: EXPECTED,
    },
    {
      code: 'export * from "./vector";\nexport * from "./arc";\nexport * from "./angle";\n',
      errors: [{ messageId: "outOfSync" }],
      filename: BARREL,
      output: EXPECTED,
    },
    {
      code: 'export { a } from "./vector";',
      errors: [{ messageId: "onlyReexports" }],
      filename: BARREL,
    },
    { code: "export const A = 1;", errors: [{ messageId: "onlyReexports" }], filename: BARREL },
  ],
  valid: [{ code: EXPECTED, filename: BARREL }],
});

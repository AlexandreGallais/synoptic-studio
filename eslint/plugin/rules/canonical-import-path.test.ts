import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

import { RuleTester } from "@typescript-eslint/rule-tester";

import { CANONICAL_IMPORT_PATH_RULE } from "./canonical-import-path";

/** Fixture tree: two layers, one with a sub-folder; every folder has a barrel. */
const ROOT = mkdtempSync(join(tmpdir(), "canonical-import-"));

/** Files of the fixture tree, relative to its root. */
const FILES = [
  "src/index.ts",
  "src/geometry/index.ts",
  "src/geometry/vector.ts",
  "src/geometry/arc/index.ts",
  "src/geometry/arc/fillet.ts",
  "src/model/index.ts",
  "src/model/shape.ts",
  "src/model/port.ts",
];

for (const file of FILES) {
  mkdirSync(dirname(join(ROOT, file)), { recursive: true });
  writeFileSync(join(ROOT, file), "export {};\n");
}

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Importer in the `model` layer. */
const SHAPE = join(ROOT, "src/model/shape.ts");
/** Importer at the root of the `GEOMETRY` layer. */
const GEOMETRY = join(ROOT, "src/geometry/vector.ts");
/** Importer in a sub-folder of the `geometry` layer. */
const FILLET = join(ROOT, "src/geometry/arc/fillet.ts");

TESTER.run("canonical-import-path", CANONICAL_IMPORT_PATH_RULE, {
  invalid: [
    {
      code: 'import "./port.ts";',
      errors: [{ messageId: "notCanonical" }],
      filename: SHAPE,
      output: 'import "./port";',
    },
    {
      code: 'import "../geometry/index";',
      errors: [{ messageId: "notCanonical" }],
      filename: SHAPE,
      output: 'import "../geometry";',
    },
    {
      code: 'import "../geometry/arc/fillet";',
      errors: [{ messageId: "notCanonical" }],
      filename: SHAPE,
      output: 'import "../geometry";',
    },
    {
      code: 'export * from "./arc/fillet";',
      errors: [{ messageId: "notCanonical" }],
      filename: GEOMETRY,
      output: 'export * from "./arc";',
    },
    { code: 'import "../index";', errors: [{ messageId: "ancestorBarrel" }], filename: GEOMETRY },
    { code: 'import ".";', errors: [{ messageId: "ancestorBarrel" }], filename: GEOMETRY },
  ],
  valid: [
    { code: 'import "./port";', filename: SHAPE },
    { code: 'import "../geometry";', filename: SHAPE },
    { code: 'import "./arc";', filename: GEOMETRY },
    { code: 'import "../vector";', filename: FILLET },
    { code: 'import "node:fs";', filename: SHAPE },
    { code: 'import "./missing";', filename: SHAPE },
  ],
});

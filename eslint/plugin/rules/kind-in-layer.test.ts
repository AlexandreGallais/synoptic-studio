import { join } from "node:path";

import { RuleTester } from "@typescript-eslint/rule-tester";

import { KIND_IN_LAYER_RULE } from "./kind-in-layer";

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Repository root of the virtual files. */
const ROOT = "/project";
/** Rule options shared by the cases. */
const OPTIONS = [
  { layers: [{ kinds: ["math"], name: "math", path: "src/math" }], root: ROOT },
] as const;
/** File inside the `math` layer. */
const IN_MATH = join(ROOT, "src/math/vector.ts");
/** File OUTSIDE every layer. */
const OUTSIDE = join(ROOT, "src/other/vector.ts");

TESTER.run("kind-in-layer", KIND_IN_LAYER_RULE, {
  invalid: [
    {
      code: "/** @kind procedure */\nfunction f(): void {}",
      errors: [{ messageId: "forbidden" }],
      filename: IN_MATH,
      options: OPTIONS,
    },
  ],
  valid: [
    { code: "/** @kind math */\nfunction f(): void {}", filename: IN_MATH, options: OPTIONS },
    { code: "/** @kind procedure */\nfunction f(): void {}", filename: OUTSIDE, options: OPTIONS },
  ],
});

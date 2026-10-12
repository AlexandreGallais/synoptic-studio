import { RuleTester } from "@typescript-eslint/rule-tester";

import { REQUIRE_KIND_RULE } from "./require-kind";

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Rule options shared by the cases. */
const OPTIONS = [{ kinds: ["math", "procedure"] }] as const;

TESTER.run("require-kind", REQUIRE_KIND_RULE, {
  invalid: [
    { code: "function f(): void {}", errors: [{ messageId: "missing" }], options: OPTIONS },
    {
      code: "/** Doc. */\nexport function f(): void {}",
      errors: [{ messageId: "missing" }],
      options: OPTIONS,
    },
    {
      code: "/**\n * Doc.\n *\n * @kind math\n * @kind procedure\n */\nfunction f(): void {}",
      errors: [{ messageId: "multiple" }],
      options: OPTIONS,
    },
    {
      code: "/** @kind magic */\nfunction f(): void {}",
      errors: [{ messageId: "unknown" }],
      options: OPTIONS,
    },
    {
      code: "/** @kind */\nfunction f(): void {}",
      errors: [{ messageId: "unknown" }],
      options: OPTIONS,
    },
  ],
  valid: [
    { code: "/** @kind math */\nfunction f(): void {}", options: OPTIONS },
    { code: "/** @kind procedure */\nexport function f(): void {}", options: OPTIONS },
    {
      code: "/** @kind math */\nfunction f(): void {\n  function inner(): void {}\n}",
      options: OPTIONS,
    },
    { code: "const f = (): number => 1;", options: OPTIONS },
  ],
});

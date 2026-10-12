import { RuleTester } from "@typescript-eslint/rule-tester";

import { KIND_NAMING_RULE } from "./kind-naming";

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Rule options shared by the cases. */
const OPTIONS = [
  {
    formatPattern: "^(?:to[A-Z]|[a-z][A-Za-z0-9]*To[A-Z])",
    predicatePrefixes: ["is", "can"],
    procedureVerbs: ["render", "mount"],
    vaguePrefixes: ["get", "compute"],
  },
] as const;

TESTER.run("kind-naming", KIND_NAMING_RULE, {
  invalid: [
    {
      code: "/** @kind procedure */\nfunction pathElement(): void {}",
      errors: [{ messageId: "procedureVerb" }],
      options: OPTIONS,
    },
    {
      code: "/** @kind format */\nfunction svgPath(): string { return ''; }",
      errors: [{ messageId: "formatName" }],
      options: OPTIONS,
    },
    {
      code: "/** @kind math */\nfunction getDot(): number { return 0; }",
      errors: [{ messageId: "vagueName" }],
      options: OPTIONS,
    },
    {
      code: "/** @kind domain */\nfunction connectable(): boolean { return true; }",
      errors: [{ messageId: "predicateName" }],
      options: OPTIONS,
    },
    {
      code: "/** @kind domain */\nfunction isPort(): number { return 0; }",
      errors: [{ messageId: "predicateReturn" }],
      options: OPTIONS,
    },
  ],
  valid: [
    { code: "/** @kind procedure */\nfunction renderShape(): void {}", options: OPTIONS },
    { code: "/** @kind format */\nfunction pathToSvg(): string { return ''; }", options: OPTIONS },
    { code: "/** @kind format */\nfunction toSvg(): string { return ''; }", options: OPTIONS },
    { code: "/** @kind math */\nfunction dot(): number { return 0; }", options: OPTIONS },
    {
      code: "/** @kind domain */\nfunction canConnect(): boolean { return true; }",
      options: OPTIONS,
    },
    {
      code: "/** @kind domain */\nfunction isPort(x: unknown): x is string { return true; }",
      options: OPTIONS,
    },
  ],
});

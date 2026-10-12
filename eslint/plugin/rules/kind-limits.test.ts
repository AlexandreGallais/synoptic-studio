import { RuleTester } from "@typescript-eslint/rule-tester";

import { KIND_LIMITS_RULE } from "./kind-limits";

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();

/** Generous limits; each case tightens only the metric it checks. */
const LOOSE = { maxComplexity: 10, maxDepth: 10, maxLines: 100, maxStatements: 100 };

/**
 * Rule options with the `math` limits overridden.
 *
 * @param overrides - limits to tighten
 * @returns options of `local/kind-limits`
 */
function mathLimits(
  overrides: Readonly<Partial<typeof LOOSE>>,
): [{ limits: Record<string, typeof LOOSE> }] {
  return [{ limits: { math: { ...LOOSE, ...overrides } } }];
}

TESTER.run("kind-limits", KIND_LIMITS_RULE, {
  invalid: [
    {
      // Complexity 3: base 1 + `if` + `&&`.
      code: "/** @kind math */\nfunction f(a: boolean, b: boolean): number {\n  if (a && b) return 1;\n  return 0;\n}",
      errors: [
        { data: { actual: 3, kind: "math", max: 2, metric: "complexity" }, messageId: "exceeded" },
      ],
      options: mathLimits({ maxComplexity: 2 }),
    },
    {
      // Complexity 4: base 1 + `?:` + `??` + `case 1` (the default case adds nothing).
      code: "/** @kind math */\nfunction f(a?: number): number {\n  switch (a ?? 0) { case 1: return a ? 1 : 2; default: return 0; }\n}",
      errors: [
        { data: { actual: 4, kind: "math", max: 3, metric: "complexity" }, messageId: "exceeded" },
      ],
      options: mathLimits({ maxComplexity: 3 }),
    },
    {
      // Depth 2: `if` inside `if`.
      code: "/** @kind math */\nfunction f(a: boolean): void {\n  if (a) { if (a) { } }\n}",
      errors: [
        { data: { actual: 2, kind: "math", max: 1, metric: "depth" }, messageId: "exceeded" },
      ],
      options: mathLimits({ maxDepth: 1 }),
    },
    {
      // 5 code lines: the blank line and the comment-only line are skipped.
      code: "/** @kind math */\nfunction f(): number {\n\n  // note\n  const a = 1;\n  const b = 2;\n  return a + b;\n}",
      errors: [
        { data: { actual: 5, kind: "math", max: 4, metric: "lines" }, messageId: "exceeded" },
      ],
      options: mathLimits({ maxLines: 4 }),
    },
    {
      // 3 statements: two declarations and a return.
      code: "/** @kind math */\nfunction f(): number {\n  const a = 1;\n  const b = 2;\n  return a + b;\n}",
      errors: [
        { data: { actual: 3, kind: "math", max: 2, metric: "statements" }, messageId: "exceeded" },
      ],
      options: mathLimits({ maxStatements: 2 }),
    },
  ],
  valid: [
    {
      code: "/** @kind math */\nfunction f(a: number): number {\n  return a > 0 ? a : -a;\n}",
      options: mathLimits({ maxComplexity: 2 }),
    },
    {
      code: "/** @kind procedure */\nfunction f(a: boolean): void {\n  if (a) { if (a) { } }\n}",
      options: mathLimits({ maxDepth: 1 }),
    },
    {
      code: "function f(a: boolean): void {\n  if (a) { if (a) { } }\n}",
      options: mathLimits({ maxDepth: 1 }),
    },
  ],
});

import { ESLintUtils } from "@typescript-eslint/utils";

/** Rule factory of the local plugin; each rule is documented in docs/conventions/lint.md. */
export const createRule = ESLintUtils.RuleCreator(
  (name) => `docs/conventions/lint.md#local${name}`,
);

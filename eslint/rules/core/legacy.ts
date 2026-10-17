import type { TSESLint } from "@typescript-eslint/utils";

/** Dangerous or obsolete language features. */
export const CORE_LEGACY: TSESLint.FlatConfig.Config = {
  name: "core/legacy",
  rules: {
    "no-alert": "error",
    "no-caller": "error",
    "no-console": "error",
    "no-debugger": "error",
    "no-eval": "error",
    // Superseded by the type-aware @typescript-eslint/no-implied-eval.
    "no-implied-eval": "off",
    "no-irregular-whitespace": [
      "error",
      { skipComments: false, skipRegExps: false, skipStrings: false, skipTemplates: false },
    ],
    "no-multi-str": "error",
    "no-new-func": "error",
    "no-obj-calls": "error",
    // Enums, `for…in`, getters and setters, dynamic imports: Run by the project's linter (tools/lint, EN-075).
    "no-restricted-syntax": "off",
    "no-script-url": "error",
    "no-with": "error",
  },
};

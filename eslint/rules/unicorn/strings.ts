import type { TSESLint } from "@typescript-eslint/utils";

/** Strings, templates and regular expressions. */
export const UNICORN_STRINGS: TSESLint.FlatConfig.Config = {
  name: "unicorn/strings",
  rules: {
    "unicorn/consistent-template-literal-escape": "error",
    "unicorn/escape-case": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-incorrect-template-string-interpolation": "off",
    // Run by the project's linter (tools/lint, EN-095).
    "unicorn/no-invalid-character-comparison": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-unnecessary-string-trim": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-unsafe-string-replacement": "off",
    // Run by oxlint as ESLint's own no-useless-concat (EN-095).
    "unicorn/no-useless-concat": "off",
    "unicorn/no-useless-template-literals": "error",
    "unicorn/prefer-code-point": "error",
    "unicorn/prefer-escaped-irregular-whitespace": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-literal-ascii": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-regexp-escape": "off",
    "unicorn/prefer-regexp-test": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-short-escape-sequences": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-single-replace": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-split-limit": "off",
    "unicorn/prefer-string-match-all": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-string-pad-start-end": "off",
    "unicorn/prefer-string-raw": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-string-repeat": "off",
    "unicorn/prefer-string-replace-all": "error",
    "unicorn/prefer-string-slice": "error",
    "unicorn/prefer-string-starts-ends-with": "error",
    "unicorn/prefer-string-trim-start-end": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-unicode-code-point-escapes": "off",
    "unicorn/string-content": "error",
  },
};

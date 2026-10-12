import type { TSESLint } from "@typescript-eslint/utils";

/** Strings, templates and regular expressions. */
export const UNICORN_STRINGS: TSESLint.FlatConfig.Config = {
  name: "unicorn/strings",
  rules: {
    "unicorn/consistent-template-literal-escape": "error",
    "unicorn/escape-case": "error",
    "unicorn/no-incorrect-template-string-interpolation": "error",
    "unicorn/no-invalid-character-comparison": "error",
    "unicorn/no-unnecessary-string-trim": "error",
    "unicorn/no-unsafe-string-replacement": "error",
    "unicorn/no-useless-concat": "error",
    "unicorn/no-useless-template-literals": "error",
    "unicorn/prefer-code-point": "error",
    "unicorn/prefer-escaped-irregular-whitespace": "error",
    "unicorn/prefer-literal-ascii": "error",
    "unicorn/prefer-regexp-escape": "error",
    "unicorn/prefer-regexp-test": "error",
    "unicorn/prefer-short-escape-sequences": "error",
    "unicorn/prefer-single-replace": "error",
    "unicorn/prefer-split-limit": "error",
    "unicorn/prefer-string-match-all": "error",
    "unicorn/prefer-string-pad-start-end": "error",
    "unicorn/prefer-string-raw": "error",
    "unicorn/prefer-string-repeat": "error",
    "unicorn/prefer-string-replace-all": "error",
    "unicorn/prefer-string-slice": "error",
    "unicorn/prefer-string-starts-ends-with": "error",
    "unicorn/prefer-string-trim-start-end": "error",
    "unicorn/prefer-unicode-code-point-escapes": "error",
    "unicorn/string-content": "error",
  },
};

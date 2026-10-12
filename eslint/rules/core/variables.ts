import { RESTRICTED_GLOBALS } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Declarations, scope and assignment of variables. */
export const CORE_VARIABLES: TSESLint.FlatConfig.Config = {
  name: "core/variables",
  rules: {
    "block-scoped-var": "error",
    // Superseded by the type-aware @typescript-eslint/init-declarations.
    "init-declarations": "off",
    "no-const-assign": "error",
    "no-delete-var": "error",
    "no-global-assign": "error",
    "no-implicit-globals": "error",
    "no-label-var": "error",
    // Superseded by the type-aware @typescript-eslint/no-redeclare.
    "no-redeclare": "off",
    // Confusing browser globals; pure layers add DOM globals (scopes/pure-layers.ts).
    "no-restricted-globals": ["error", ...RESTRICTED_GLOBALS],
    // Superseded by the type-aware @typescript-eslint/no-shadow.
    "no-shadow": "off",
    "no-shadow-restricted-names": ["error", { reportGlobalThis: true }],
    "no-unassigned-vars": "error",
    // TypeScript resolves identifiers; the core rule reports false positives on types (typescript-eslint FAQ).
    "no-undef": "off",
    "no-undef-init": "error",
    // `undefined` is the project's absence value (unicorn/no-null); forbidding the identifier would force `void 0`.
    "no-undefined": "off",
    // Superseded by unused-imports/no-unused-vars, which also autofixes imports.
    "no-unused-vars": "off",
    // Superseded by the type-aware @typescript-eslint/no-use-before-define.
    "no-use-before-define": "off",
    "no-useless-assignment": "error",
    "no-var": "error",
    "one-var": ["error", "never"],
    "prefer-const": ["error", { destructuring: "all" }],
    "sort-vars": "error",
    "vars-on-top": "error",
  },
};

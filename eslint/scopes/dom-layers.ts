import { DOM_LAYER_FILES } from "../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/**
 * Imperative shell (render, interaction, playground): side effects are the point.
 *
 * Data stays immutable through types and `no-param-reassign`; only the statement-level purity
 * rules are relaxed. See ADR-0014 (functional core, imperative shell).
 */
export const DOM_LAYERS: TSESLint.FlatConfig.Config = {
  files: DOM_LAYER_FILES,
  name: "scopes/dom-layers",
  rules: {
    "functional/functional-parameters": [
      "error",
      { allowArgumentsKeyword: false, allowRestParameter: true, enforceParameterCount: false },
    ],
    "functional/immutable-data": "off",
    "functional/no-conditional-statements": "off",
    "functional/no-expression-statements": "off",
    "functional/no-loop-statements": "off",
    "functional/no-return-void": "off",
    "functional/no-throw-statements": "off",
    "functional/no-try-statements": "off",
    "functional/prefer-immutable-types": "off",
    "functional/type-declaration-immutability": "off",
  },
};

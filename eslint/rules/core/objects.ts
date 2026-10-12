import { RESTRICTED_PROPERTIES } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Object and array literals, properties and destructuring. */
export const CORE_OBJECTS: TSESLint.FlatConfig.Config = {
  name: "core/objects",
  rules: {
    "accessor-pairs": ["error", { enforceForClassMembers: true }],
    "array-callback-return": ["error", { checkForEach: true }],
    // Superseded by the type-aware @typescript-eslint/dot-notation.
    "dot-notation": "off",
    "grouped-accessor-pairs": ["error", "getBeforeSet"],
    // Superseded by the type-aware @typescript-eslint/no-array-constructor.
    "no-array-constructor": "off",
    "no-dupe-keys": "error",
    "no-empty-pattern": "error",
    "no-extend-native": "error",
    "no-iterator": "error",
    "no-new-wrappers": "error",
    "no-object-constructor": "error",
    "no-proto": "error",
    "no-prototype-builtins": "error",
    "no-restricted-properties": ["error", ...RESTRICTED_PROPERTIES],
    "no-sparse-arrays": "error",
    "no-useless-computed-key": ["error", { enforceForClassMembers: true }],
    "no-useless-rename": "error",
    "object-shorthand": ["error", "always", { avoidQuotes: true }],
    // Superseded by the type-aware @typescript-eslint/prefer-destructuring.
    "prefer-destructuring": "off",
    "prefer-object-has-own": "error",
    "prefer-object-spread": "error",
    // Semantic order beats alphabetical order (start before end, min before max).
    "sort-keys": "off",
    "symbol-description": "error",
  },
};

import { GLOBAL_FUNCTION_CEILING, MAX_FILE_LINES } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Global ceilings on size and nesting; tighter per-kind limits live in local/kind-limits. */
export const CORE_COMPLEXITY: TSESLint.FlatConfig.Config = {
  name: "core/complexity",
  rules: {
    complexity: ["error", { max: GLOBAL_FUNCTION_CEILING.maxComplexity }],
    "max-classes-per-file": ["error", 1],
    "max-depth": ["error", { max: GLOBAL_FUNCTION_CEILING.maxDepth }],
    "max-lines": ["error", { max: MAX_FILE_LINES, skipBlankLines: true, skipComments: true }],
    "max-lines-per-function": [
      "error",
      {
        IIFEs: true,
        max: GLOBAL_FUNCTION_CEILING.maxLines,
        skipBlankLines: true,
        skipComments: true,
      },
    ],
    "max-nested-callbacks": ["error", { max: GLOBAL_FUNCTION_CEILING.maxNestedCallbacks }],
    // Superseded by the type-aware @typescript-eslint/max-params.
    "max-params": "off",
    "max-statements": ["error", { max: GLOBAL_FUNCTION_CEILING.maxStatements }],
    "no-nested-ternary": "error",
  },
};

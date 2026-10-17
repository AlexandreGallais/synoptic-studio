import { LIBRARY_DOCUMENTED_CONTEXTS, LIBRARY_FILES } from "../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Library code: documented type members (its default and optional parameters: tools/lint, ADR-0016). */
export const LIBRARY_SIGNATURES: TSESLint.FlatConfig.Config = {
  files: LIBRARY_FILES,
  name: "scopes/library-signatures",
  rules: {
    // Type members are documented too: `tools/api-pages` fails the docs build on an undocumented property.
    "jsdoc/require-jsdoc": [
      "error",
      {
        contexts: LIBRARY_DOCUMENTED_CONTEXTS,
        enableFixer: false,
        publicOnly: false,
        require: {
          ArrowFunctionExpression: false,
          ClassDeclaration: true,
          ClassExpression: true,
          FunctionDeclaration: true,
          FunctionExpression: false,
          MethodDefinition: true,
        },
      },
    ],
  },
};

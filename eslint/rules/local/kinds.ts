import {
  FORMAT_NAME_PATTERN,
  KIND_LIMITS,
  KIND_NAMES,
  LAYERS,
  PREDICATE_PREFIXES,
  PROCEDURE_VERBS,
  PROJECT_ROOT,
  VAGUE_PREFIXES,
} from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Function kinds (ADR-0011): presence, layer, size and naming. */
export const LOCAL_KINDS: TSESLint.FlatConfig.Config = {
  name: "local/kinds",
  rules: {
    "local/kind-in-layer": ["error", { layers: LAYERS, root: PROJECT_ROOT }],
    "local/kind-limits": ["error", { limits: KIND_LIMITS }],
    "local/kind-naming": [
      "error",
      {
        formatPattern: FORMAT_NAME_PATTERN,
        predicatePrefixes: PREDICATE_PREFIXES,
        procedureVerbs: PROCEDURE_VERBS,
        vaguePrefixes: VAGUE_PREFIXES,
      },
    ],
    "local/require-kind": ["error", { kinds: KIND_NAMES }],
  },
};

import { KIND_NAMES, PROJECT_ROOT, REFERENCE_SOURCES } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** `@see` references: required on every kind, existing and verified. */
export const LOCAL_DOCUMENTATION: TSESLint.FlatConfig.Config = {
  name: "local/documentation",
  rules: {
    "local/see-references": [
      "error",
      { requiredKinds: KIND_NAMES, root: PROJECT_ROOT, sources: REFERENCE_SOURCES },
    ],
  },
};

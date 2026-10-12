import { IMPORTS_DEFAULT_EXPORTS } from "./default-exports";
import { IMPORTS_DEPENDENCIES } from "./dependencies";
import { IMPORTS_MODULE_SYSTEMS } from "./module-systems";
import { IMPORTS_RESOLUTION } from "./resolution";
import { IMPORTS_STYLE } from "./style";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `imports` rule theme, in file order. */
export const IMPORTS_RULES: readonly TSESLint.FlatConfig.Config[] = [
  IMPORTS_DEFAULT_EXPORTS,
  IMPORTS_DEPENDENCIES,
  IMPORTS_MODULE_SYSTEMS,
  IMPORTS_RESOLUTION,
  IMPORTS_STYLE,
];

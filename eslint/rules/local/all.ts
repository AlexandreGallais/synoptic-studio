import { LOCAL_DOCUMENTATION } from "./documentation";
import { LOCAL_KINDS } from "./kinds";
import { LOCAL_MODULES } from "./modules";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `local` rule theme, in file order. */
export const LOCAL_RULES: readonly TSESLint.FlatConfig.Config[] = [
  LOCAL_DOCUMENTATION,
  LOCAL_KINDS,
  LOCAL_MODULES,
];

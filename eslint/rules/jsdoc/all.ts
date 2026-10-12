import { JSDOC_CONTENT } from "./content";
import { JSDOC_LAYOUT } from "./layout";
import { JSDOC_PRESENCE } from "./presence";
import { JSDOC_TYPES } from "./types";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `jsdoc` rule theme, in file order. */
export const JSDOC_RULES: readonly TSESLint.FlatConfig.Config[] = [
  JSDOC_CONTENT,
  JSDOC_LAYOUT,
  JSDOC_PRESENCE,
  JSDOC_TYPES,
];

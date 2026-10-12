import { TYPESCRIPT_CLASSES } from "./classes";
import { TYPESCRIPT_EXTENSIONS } from "./extensions";
import { TYPESCRIPT_NAMING } from "./naming";
import { TYPESCRIPT_PROMISES } from "./promises";
import { TYPESCRIPT_SIGNATURES } from "./signatures";
import { TYPESCRIPT_STRICTNESS } from "./strictness";
import { TYPESCRIPT_TYPE_DECLARATIONS } from "./type-declarations";
import { TYPESCRIPT_TYPE_IMPORTS } from "./type-imports";
import { TYPESCRIPT_TYPE_SAFETY } from "./type-safety";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `typescript` rule theme, in file order. */
export const TYPESCRIPT_RULES: readonly TSESLint.FlatConfig.Config[] = [
  TYPESCRIPT_CLASSES,
  TYPESCRIPT_EXTENSIONS,
  TYPESCRIPT_NAMING,
  TYPESCRIPT_PROMISES,
  TYPESCRIPT_SIGNATURES,
  TYPESCRIPT_STRICTNESS,
  TYPESCRIPT_TYPE_DECLARATIONS,
  TYPESCRIPT_TYPE_IMPORTS,
  TYPESCRIPT_TYPE_SAFETY,
];

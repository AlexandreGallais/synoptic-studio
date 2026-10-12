import { MARKDOWN_STRUCTURE } from "./structure";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `markdown` rule theme, in file order. */
export const MARKDOWN_RULES: readonly TSESLint.FlatConfig.Config[] = [MARKDOWN_STRUCTURE];

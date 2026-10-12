import { CORE_ASYNC } from "./async";
import { CORE_CLASSES } from "./classes";
import { CORE_COMMENTS } from "./comments";
import { CORE_COMPLEXITY } from "./complexity";
import { CORE_CONTROL_FLOW } from "./control-flow";
import { CORE_ERRORS } from "./errors";
import { CORE_FUNCTIONS } from "./functions";
import { CORE_LEGACY } from "./legacy";
import { CORE_MODULES } from "./modules";
import { CORE_NAMING } from "./naming";
import { CORE_OBJECTS } from "./objects";
import { CORE_OPERATORS } from "./operators";
import { CORE_REGEX } from "./regex";
import { CORE_VARIABLES } from "./variables";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `core` rule theme, in file order. */
export const CORE_RULES: readonly TSESLint.FlatConfig.Config[] = [
  CORE_ASYNC,
  CORE_CLASSES,
  CORE_COMMENTS,
  CORE_COMPLEXITY,
  CORE_CONTROL_FLOW,
  CORE_ERRORS,
  CORE_FUNCTIONS,
  CORE_LEGACY,
  CORE_MODULES,
  CORE_NAMING,
  CORE_OBJECTS,
  CORE_OPERATORS,
  CORE_REGEX,
  CORE_VARIABLES,
];

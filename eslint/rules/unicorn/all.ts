import { UNICORN_ARRAYS } from "./arrays";
import { UNICORN_COLLECTIONS } from "./collections";
import { UNICORN_CONTROL_FLOW } from "./control-flow";
import { UNICORN_DOM } from "./dom";
import { UNICORN_ERRORS } from "./errors";
import { UNICORN_FUNCTIONS } from "./functions";
import { UNICORN_ITERATION } from "./iteration";
import { UNICORN_MISC } from "./misc";
import { UNICORN_MODULES } from "./modules";
import { UNICORN_NAMING } from "./naming";
import { UNICORN_NUMBERS } from "./numbers";
import { UNICORN_OBJECTS } from "./objects";
import { UNICORN_PLATFORM } from "./platform";
import { UNICORN_PROMISES } from "./promises";
import { UNICORN_STRINGS } from "./strings";
import { UNICORN_WEB } from "./web";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `unicorn` rule theme, in file order. */
export const UNICORN_RULES: readonly TSESLint.FlatConfig.Config[] = [
  UNICORN_ARRAYS,
  UNICORN_COLLECTIONS,
  UNICORN_CONTROL_FLOW,
  UNICORN_DOM,
  UNICORN_ERRORS,
  UNICORN_FUNCTIONS,
  UNICORN_ITERATION,
  UNICORN_MISC,
  UNICORN_MODULES,
  UNICORN_NAMING,
  UNICORN_NUMBERS,
  UNICORN_OBJECTS,
  UNICORN_PLATFORM,
  UNICORN_PROMISES,
  UNICORN_STRINGS,
  UNICORN_WEB,
];

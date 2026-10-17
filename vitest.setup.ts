// Binds the typed ESLint RuleTester to Vitest (Vitest globals are disabled in this project), for the
// tests of the ESLint rules only: the `rules` project of vite.config.ts (EN-091).
// Vitest's hooks return values the RuleTester ignores: thin wrappers make that explicit.

import { RuleTester } from "@typescript-eslint/rule-tester";
import { afterAll, describe, it } from "vitest";

RuleTester.afterAll = (callback): void => {
  afterAll(callback);
};

RuleTester.describe = (text, callback): void => {
  describe(text, callback);
};

RuleTester.it = (text, callback): void => {
  it(text, callback);
};

RuleTester.itOnly = (text, callback): void => {
  it.only(text, callback);
};

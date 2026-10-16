import { join } from "node:path";

import { ESLint } from "eslint";
import { describe, expect, it } from "vitest";

import { RESTRICTED_SYNTAX } from "./identifiers";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..", "..");

/** `no-restricted-syntax` alone, with the project's selectors, on plain JavaScript. */
const RESTRICTED = new ESLint({
  cwd: ROOT,
  overrideConfig: { rules: { "no-restricted-syntax": ["error", ...RESTRICTED_SYNTAX] } },
  overrideConfigFile: true,
});

/** The project's ESLint, read from its configuration. */
const PROJECT = new ESLint({ cwd: ROOT });

/** Start of the message of the fold-spread selector. */
const FOLD_SPREAD_MESSAGE = "A fold that spreads its accumulator";

/**
 * Lines that `no-restricted-syntax` reports in a piece of code.
 *
 * @param code - JavaScript source
 * @returns the line of each report
 */
async function reportedLines(code: string): Promise<readonly number[]> {
  const [result] = await RESTRICTED.lintText(code);

  return result?.messages.map((message) => message.line) ?? [];
}

/**
 * Messages of the `no-restricted-syntax` selectors that the project's configuration gives a file.
 *
 * @param file - path from the repository root
 * @returns each selector's message
 */
async function restrictedMessages(file: string): Promise<readonly string[]> {
  const config: unknown = await PROJECT.calculateConfigForFile(file);
  const rules: unknown = Reflect.get(new Object(config), "rules");
  const entry: unknown = Reflect.get(new Object(rules), "no-restricted-syntax");

  return (Array.isArray(entry) ? entry : []).map((option: unknown) =>
    String(Reflect.get(new Object(option), "message")),
  );
}

describe("no spread in a fold (EN-061)", () => {
  it.each([
    {
      code: "[1].reduce((all, x) => [...all, x], []);",
      title: "an array spread as the arrow's body",
    },
    {
      code: "[1].reduce((all, x) => ({ ...all, [x]: x }), {});",
      title: "an object spread as the arrow's body",
    },
    {
      code: "[1].reduce(function (all, x) { return [...all, x]; }, []);",
      title: "a spread returned from a block",
    },
    { code: "[1].reduceRight((all, x) => [...all, x], []);", title: "a spread in reduceRight" },
  ])("[F18.AC18] reports $title", async ({ code }) => {
    expect(await reportedLines(code)).toEqual([1]);
  });

  it.each([
    { code: "[1].reduce((sum, x) => sum + x, 0);", title: "a sum" },
    { code: "[[1]].map((row) => [...row, 0]);", title: "a spread in a map" },
    {
      code: "[[1]].reduce((top, row) => Math.max(top, ...row), 0);",
      title: "a spread passed to a call",
    },
    {
      code: "[1].reduce((sum, x) => sum + x, [...[0]].length);",
      title: "a spread in the initial value",
    },
  ])("lets $title pass", async ({ code }) => {
    expect(await reportedLines(code)).toEqual([]);
  });

  it.each(["src/math/vector/norm.ts", "playground/points/index.ts", "tests/folders.test.ts"])(
    "[F18.AC18] applies the selector to %s",
    async (file) => {
      const messages = await restrictedMessages(file);

      expect(messages.filter((message) => message.startsWith(FOLD_SPREAD_MESSAGE))).toHaveLength(1);
    },
  );
});

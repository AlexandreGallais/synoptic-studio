import { capitalized } from "./capitalized";
import { caseWords } from "./case-words";

import type { TextCase } from "../text";

/** How each case writes one line (Q36). */
const LINE_IN_CASE: Readonly<Record<TextCase, (line: string) => string>> = {
  camel: (line) =>
    caseWords(line.toLowerCase())
      .map((word, index) => (index === 0 ? word : capitalized(word)))
      .join(""),
  constant: (line) => caseWords(line.toUpperCase()).join("_"),
  kebab: (line) => caseWords(line.toLowerCase()).join("-"),
  lower: (line) => line.toLowerCase(),
  none: (line) => line,
  pascal: (line) =>
    caseWords(line.toLowerCase())
      .map((word) => capitalized(word))
      .join(""),
  sentence: (line) => capitalized(line),
  snake: (line) => caseWords(line.toLowerCase()).join("_"),
  title: (line) =>
    line
      .split(" ")
      .map((word) => capitalized(word))
      .join(" "),
  upper: (line) => line.toUpperCase(),
};

/**
 * The content of a text written in a case, line by line, its line breaks kept (Q36): upper,
 * lower, its first character capitalized (sentence), the first character after each space
 * capitalized (title), or its words, cut at spaces, hyphens and underscores, joined in camel,
 * Pascal, kebab, snake or constant case.
 *
 * @kind domain
 * @param content - what the designer typed, a line feed between two lines
 * @param textCase - the case to write it in
 * @returns the content in that case
 * @see docs/domain/shapes.md#text
 */
export function textInCase(content: string, textCase: TextCase): string {
  return content
    .split("\n")
    .map((line) => LINE_IN_CASE[textCase](line))
    .join("\n");
}

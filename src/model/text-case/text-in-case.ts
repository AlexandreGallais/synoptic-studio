import { capitalized } from "./capitalized";
import { caseWords } from "./case-words";

import type { TextCase } from "../text";

/** A word of the title case: a run of characters other than white space. */
const WORD = /\S+/gu;

/** A line: a run of characters other than a carriage return or a line feed. */
const LINE = /[^\n\r]+/gu;

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
  snake: (line) => caseWords(line.toLowerCase()).join("_"),
  title: (line) => line.replaceAll(WORD, (word) => capitalized(word)),
  upper: (line) => line.toUpperCase(),
};

/**
 * The content of a text written in a case, line by line, its line breaks kept (Q36): upper,
 * lower, the first letter of each run of characters between white space capitalized (title), or
 * its words, cut at spaces, hyphens and underscores, joined in camel, Pascal, kebab, snake or
 * constant case.
 *
 * @kind domain
 * @param content - what the designer typed, a line break (CR LF, CR or LF) between two lines
 * @param textCase - the case to write it in
 * @returns the content in that case
 * @see RULE-shapes-text
 */
export function textInCase(content: string, textCase: TextCase): string {
  return content.replaceAll(LINE, (line) => LINE_IN_CASE[textCase](line));
}

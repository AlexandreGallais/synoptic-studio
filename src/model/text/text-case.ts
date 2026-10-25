/**
 * How the library writes the case of a text (Q36, `RULE-shapes-text`): kept as typed,
 * upper, lower, every word capitalized (title), or one of the programmers' cases camel, Pascal,
 * kebab, snake and constant; sentence case was removed as of no use.
 */
export type TextCase =
  | "none"
  | "upper"
  | "lower"
  | "title"
  | "camel"
  | "pascal"
  | "kebab"
  | "snake"
  | "constant";

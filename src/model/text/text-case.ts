/**
 * How the library writes the case of a text (Q36, `docs/domain/shapes.md`, Text): kept as typed,
 * upper, lower, its first letter capitalized (sentence), every word capitalized (title), or one of
 * the programmers' cases camel, Pascal, kebab, snake and constant.
 */
export type TextCase =
  | "none"
  | "upper"
  | "lower"
  | "sentence"
  | "title"
  | "camel"
  | "pascal"
  | "kebab"
  | "snake"
  | "constant";

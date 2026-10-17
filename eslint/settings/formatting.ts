/**
 * Blank lines between statements (`@stylistic/padding-line-between-statements`).
 *
 * Later entries override earlier ones. Prettier keeps blank lines as written, so this rule
 * decides them: code reads in paragraphs (declarations, then actions, then the result).
 */
export const PADDING_LINES = [
  { blankLine: "always", next: "*", prev: ["const", "let"] },
  { blankLine: "any", next: ["const", "let"], prev: ["const", "let"] },
  { blankLine: "always", next: ["const", "let"], prev: "expression" },
  {
    blankLine: "always",
    next: "*",
    prev: ["block-like", "multiline-expression", "directive", "import"],
  },
  {
    blankLine: "always",
    next: ["block-like", "multiline-expression", "return", "export"],
    prev: "*",
  },
  { blankLine: "any", next: "import", prev: "import" },
  { blankLine: "any", next: "export", prev: "export" },
];

/** Order and grouping of import statements (`import-x/order`). */
export const IMPORT_ORDER = {
  alphabetize: { caseInsensitive: false, order: "asc", orderImportKind: "asc" },
  groups: ["builtin", "external", "internal", "parent", "sibling", "index", "object", "type"],
  "newlines-between": "always",
};

/** Import style of Node.js built-in modules (`unicorn/import-style`): named imports. */
export const NODE_IMPORT_STYLES = {
  "node:fs": { named: true },
  "node:os": { named: true },
  "node:path": { named: true },
  "node:url": { named: true },
};

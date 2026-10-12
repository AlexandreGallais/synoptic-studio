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

/** Order of JSDoc block tags: classification, contract, references. */
export const JSDOC_TAG_SEQUENCE = [
  { tags: ["kind"] },
  { tags: ["template", "param", "returns", "yields", "throws", "rejects"] },
  { tags: ["see", "deprecated"] },
];

/** Declarations requiring a JSDoc block besides functions: types and module-level constants. */
export const DOCUMENTED_CONTEXTS = [
  "TSTypeAliasDeclaration",
  "TSInterfaceDeclaration",
  "Program > VariableDeclaration",
  "ExportNamedDeclaration[declaration.type='VariableDeclaration']",
];

/** Library declarations requiring a JSDoc block: those above plus type members (TypeDoc validation). */
export const LIBRARY_DOCUMENTED_CONTEXTS = [...DOCUMENTED_CONTEXTS, "TSPropertySignature"];

/** Fragment without final period: `@param x - abscissa of the point`. */
const TAG_FRAGMENT = {
  match: String.raw`^[\s\S]*[^.\s]\s*$`,
  message: "Tag descriptions are fragments without final period.",
};

/** JSDoc description shapes (`jsdoc/match-description`). */
export const JSDOC_DESCRIPTION_PATTERNS = {
  mainDescription: {
    match: String.raw`^[A-Z\x60][\s\S]*[.:]\s*$`,
    message: "Main descriptions are sentences: capital first letter, final period.",
  },
  tags: { param: TAG_FRAGMENT, returns: TAG_FRAGMENT },
};

/** Import style of Node.js built-in modules (`unicorn/import-style`): named imports. */
export const NODE_IMPORT_STYLES = {
  "node:fs": { named: true },
  "node:os": { named: true },
  "node:path": { named: true },
  "node:url": { named: true },
};

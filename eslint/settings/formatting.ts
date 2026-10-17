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

/** Single-letter names allowed: standard mathematical variables (x, y, r, θ written `t`…). */
export const SHORT_MATH_IDENTIFIERS = [
  "a",
  "b",
  "c",
  "d",
  "h",
  "i",
  "j",
  "k",
  "n",
  "p",
  "q",
  "r",
  "s",
  "t",
  "u",
  "v",
  "w",
  "x",
  "y",
  "z",
];

/** Names too vague to state a concept. */
export const DENIED_IDENTIFIERS = [
  "bar",
  "baz",
  "foo",
  "helper",
  "helpers",
  "info",
  "manager",
  "obj",
  "res",
  "ret",
  "stuff",
  "temp",
  "thing",
  "tmp",
  "util",
  "utils",
  "val",
];

/** Browser globals easily used by mistake, and legacy global functions. */
export const RESTRICTED_GLOBALS = [
  { message: "Use Number.isFinite.", name: "isFinite" },
  { message: "Use Number.isNaN.", name: "isNaN" },
  { message: "Use the event parameter of the listener.", name: "event" },
  { message: "Ambiguous browser global (window.name).", name: "name" },
  { message: "Ambiguous browser global (window.length).", name: "length" },
  { message: "Ambiguous browser global (window.status).", name: "status" },
  { message: "Ambiguous browser global (window.parent).", name: "parent" },
  { message: "Ambiguous browser global (window.top).", name: "top" },
];

/**
 * Globals of a host (browser or Node.js), forbidden in the functional core: it runs in any
 * ECMAScript engine and stays portable to another language (ADR-0025).
 */
export const DOM_GLOBALS = [
  "Buffer",
  "Intl",
  "__dirname",
  "__filename",
  "console",
  "crypto",
  "customElements",
  "document",
  "fetch",
  "global",
  "globalThis",
  "localStorage",
  "location",
  "module",
  "navigator",
  "process",
  "queueMicrotask",
  "require",
  "requestAnimationFrame",
  "sessionStorage",
  "setInterval",
  "setTimeout",
  "structuredClone",
  "window",
].map((name) => ({
  message: "The functional core uses ECMAScript only: no DOM, no Node.js, no host API (ADR-0025).",
  name,
}));

/** Non-deterministic properties: inputs are passed explicitly instead. */
export const RESTRICTED_PROPERTIES = [
  {
    message: "Non-deterministic: pass the value as an argument.",
    object: "Math",
    property: "random",
  },
  { message: "Non-deterministic: pass the value as an argument.", object: "Date", property: "now" },
  {
    message: "Non-deterministic: pass the value as an argument.",
    object: "performance",
    property: "now",
  },
];

/** Mutable collection types replaced by their read-only counterparts. */
export const RESTRICTED_TYPES = {
  Map: { fixWith: "ReadonlyMap", message: "Data is immutable: use ReadonlyMap." },
  Set: { fixWith: "ReadonlySet", message: "Data is immutable: use ReadonlySet." },
};

/** The callback of a fold: the first argument of `reduce` or `reduceRight`. */
const FOLD_CALLBACK =
  "CallExpression[callee.property.name=/^reduce(?:Right)?$/] > :function:first-child";

/** A spread into the array or object a callback returns, as its arrow body or from a `return`. */
const RETURNED_SPREAD = ":matches(ArrayExpression, ObjectExpression) > SpreadElement";

/** The same spread in a branch of a conditional returned (`c ? [...acc, x] : acc`). */
const BRANCH_SPREAD = `ConditionalExpression > ${RETURNED_SPREAD}`;

/**
 * A spread of the accumulator at each step of a fold (`[...acc, x]`, `{ ...acc, k: v }`); a
 * selector cannot tell the accumulator from another name, so any spread into the returned
 * literal is reported (REF-OXLINT-ACCUMULATING-SPREAD, REF-ESLINT-RESTRICTED-SYNTAX). Known
 * limit: a spread held in a variable, then returned (`const next = [...acc, x]; return next;`),
 * is not reported, a selector following no name.
 */
const FOLD_SPREAD = [
  `${FOLD_CALLBACK} > ${RETURNED_SPREAD}`,
  `${FOLD_CALLBACK} ReturnStatement > ${RETURNED_SPREAD}`,
  `${FOLD_CALLBACK} > ${BRANCH_SPREAD}`,
  `${FOLD_CALLBACK} ReturnStatement > ${BRANCH_SPREAD}`,
].join(", ");

/** Syntax outside the project's style. */
export const RESTRICTED_SYNTAX = [
  { message: "Use a union of string literals instead of an enum.", selector: "TSEnumDeclaration" },
  { message: "Iterate over Object.keys/values/entries instead.", selector: "ForInStatement" },
  {
    message: "No accessors: data is plain and readonly.",
    selector: "MethodDefinition[kind=/^(get|set)$/], Property[kind=/^(get|set)$/]",
  },
  {
    message: "Re-export without namespace: `export * from`.",
    selector: "ExportAllDeclaration[exported!=null]",
  },
  {
    message: "The dependency graph stays static: no dynamic import.",
    selector: "ImportExpression",
  },
  {
    message:
      "A fold that spreads its accumulator copies it at every step, in quadratic time: build the result with `map` or `flatMap`; never mutate the accumulator (ADR-0014).",
    selector: FOLD_SPREAD,
  },
];

/** Library syntax restrictions: every input is explicit, no default nor optional parameter (ADR-0016). */
export const LIBRARY_RESTRICTED_SYNTAX = [
  ...RESTRICTED_SYNTAX,
  {
    message:
      "No default parameter: every input is explicit; defaults live in the model (ADR-0016).",
    selector: ":function > AssignmentPattern, :function > ObjectPattern AssignmentPattern",
  },
  {
    message: "No optional parameter: every input is explicit (ADR-0016).",
    selector: ":function > Identifier[optional=true]",
  },
];

/** Leading words of boolean variables and parameters. */
const BOOLEAN_PREFIXES = ["are", "can", "did", "does", "has", "is", "should", "was", "will"];

/** `format: null` disables the casing check; the option schema of naming-convention requires `null`. */
// oxlint-disable-next-line unicorn/no-null -- required by the naming-convention option schema.
const ANY_FORMAT = null;

/** `@typescript-eslint/naming-convention` policy (docs/conventions/naming.md). */
export const NAMING_CONVENTION = [
  {
    format: ["camelCase"],
    leadingUnderscore: "forbid",
    selector: "default",
    trailingUnderscore: "forbid",
  },
  { format: ["camelCase", "PascalCase", "UPPER_CASE"], selector: "import" },
  { format: ["UPPER_CASE"], modifiers: ["const", "global"], selector: "variable" },
  {
    format: ["camelCase"],
    modifiers: ["const", "global"],
    selector: "variable",
    types: ["function"],
  },
  { format: ANY_FORMAT, modifiers: ["destructured"], selector: "variable" },
  {
    format: ["PascalCase"],
    prefix: BOOLEAN_PREFIXES,
    selector: ["parameter", "variable"],
    types: ["boolean"],
  },
  { format: ["PascalCase"], selector: "typeLike" },
  {
    format: ["camelCase", "PascalCase"],
    selector: ["objectLiteralProperty", "objectLiteralMethod"],
  },
  {
    format: ANY_FORMAT,
    modifiers: ["requiresQuotes"],
    selector: ["objectLiteralProperty", "objectLiteralMethod", "typeProperty", "typeMethod"],
  },
  {
    filter: { match: true, regex: "^IIFEs$" },
    format: ANY_FORMAT,
    selector: "objectLiteralProperty",
  },
];

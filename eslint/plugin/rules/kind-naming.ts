import { AST_NODE_TYPES } from "@typescript-eslint/utils";

import { TOP_LEVEL_FUNCTION_SELECTOR, createRule, kindOf } from "../utils";

import type { TopLevelFunction } from "../utils";

/** Naming vocabularies of `local/kind-naming`. */
type Vocabulary = {
  readonly formatPattern: string;
  readonly predicatePrefixes: readonly string[];
  readonly procedureVerbs: readonly string[];
  readonly vaguePrefixes: readonly string[];
};

/** Options of `local/kind-naming`. */
type Options = [Vocabulary];

/** Message identifiers of `local/kind-naming`. */
type MessageIds =
  "formatName" | "predicateName" | "predicateReturn" | "procedureVerb" | "vagueName";

/** Name check of one kind: the message to report when `isValid` fails. */
type KindCheck = readonly [MessageIds, (vocabulary: Vocabulary, name: string) => boolean];

/**
 * First lower-case word of a camelCase identifier.
 *
 * @param name - camelCase identifier
 * @returns its leading word, empty when the name starts with a capital
 */
function firstWord(name: string): string {
  return /^[a-z]+/u.exec(name)?.[0] ?? "";
}

/**
 * Whether a procedure name starts with an allowed imperative verb.
 *
 * @param vocabulary - naming vocabularies
 * @param name - function name
 * @returns `true` when the leading word is an allowed verb
 */
function startsWithVerb(vocabulary: Vocabulary, name: string): boolean {
  return vocabulary.procedureVerbs.includes(firstWord(name));
}

/**
 * Whether a format function name reads as a conversion.
 *
 * @param vocabulary - naming vocabularies
 * @param name - function name
 * @returns `true` when the name matches the format pattern
 */
function readsAsConversion(vocabulary: Vocabulary, name: string): boolean {
  const pattern = new RegExp(vocabulary.formatPattern, "u");

  return pattern.test(name);
}

/**
 * Whether a concept name avoids vague leading verbs (`get`, `compute`…).
 *
 * @param vocabulary - naming vocabularies
 * @param name - function name
 * @returns `true` when the leading word is not vague
 */
function namesConcept(vocabulary: Vocabulary, name: string): boolean {
  return !vocabulary.vaguePrefixes.includes(firstWord(name));
}

/** Name check of each kind. */
const KIND_CHECKS: Readonly<Record<string, KindCheck>> = {
  domain: ["vagueName", namesConcept],
  format: ["formatName", readsAsConversion],
  geometry: ["vagueName", namesConcept],
  math: ["vagueName", namesConcept],
  procedure: ["procedureVerb", startsWithVerb],
};

/**
 * Whether a function is declared as returning a boolean or a type predicate.
 *
 * @param node - function declaration
 * @returns `true` for `: boolean` and `: x is T` return annotations
 */
function returnsBoolean(node: TopLevelFunction): boolean {
  const type = node.returnType?.typeAnnotation.type;

  return type === AST_NODE_TYPES.TSBooleanKeyword || type === AST_NODE_TYPES.TSTypePredicate;
}

/**
 * Predicate-naming violation: booleans start with a predicate word, and only booleans do.
 *
 * @param vocabulary - naming vocabularies
 * @param node - function declaration
 * @returns the violated message, or `undefined` when consistent
 */
function predicateViolation(
  vocabulary: Vocabulary,
  node: TopLevelFunction,
): MessageIds | undefined {
  const isPredicateName = vocabulary.predicatePrefixes.includes(firstWord(node.id?.name ?? ""));

  if (returnsBoolean(node) === isPredicateName) {
    return undefined;
  }

  return isPredicateName ? "predicateReturn" : "predicateName";
}

/**
 * Kind-naming violation of a function.
 *
 * @param vocabulary - naming vocabularies
 * @param name - function name
 * @param kind - function kind
 * @returns the violated message, or `undefined` when the name fits the kind
 */
function kindViolation(vocabulary: Vocabulary, name: string, kind: string): MessageIds | undefined {
  const check = KIND_CHECKS[kind];

  if (check === undefined) {
    return undefined;
  }

  const [messageId, isValid] = check;

  return isValid(vocabulary, name) ? undefined : messageId;
}

/**
 * Naming violation of a function: predicate consistency first, then the rule of its kind.
 *
 * @param vocabulary - naming vocabularies
 * @param node - function declaration
 * @param kind - function kind, empty when unknown
 * @returns the violated message, or `undefined` when the name is valid
 */
function namingViolation(
  vocabulary: Vocabulary,
  node: TopLevelFunction,
  kind: string,
): MessageIds | undefined {
  return (
    predicateViolation(vocabulary, node) ?? kindViolation(vocabulary, node.id?.name ?? "", kind)
  );
}

/**
 * `local/kind-naming`: naming conventions depending on the function's `@kind`.
 *
 * @see docs/conventions/naming.md
 */
export const KIND_NAMING_RULE = createRule<Options, MessageIds>({
  create: (context, [vocabulary]) => ({
    [TOP_LEVEL_FUNCTION_SELECTOR]: (node: TopLevelFunction): void => {
      const kind = kindOf(context.sourceCode, node) ?? "";
      const messageId = namingViolation(vocabulary, node, kind);

      if (messageId === undefined || node.id === null) {
        return;
      }

      const { name } = node.id;

      context.report({ data: { kind, name, word: firstWord(name) }, messageId, node: node.id });
    },
  }),
  defaultOptions: [
    { formatPattern: ".", predicatePrefixes: [], procedureVerbs: [], vaguePrefixes: [] },
  ],
  meta: {
    docs: { description: "Enforce naming conventions depending on the function kind." },
    messages: {
      formatName:
        "`format` function `{{name}}` must read as a conversion: `xToY`, `toX`, `parseX`, `formatX`…",
      predicateName:
        "`{{name}}` returns a boolean: start its name with a predicate word (`is`, `has`, `can`…).",
      predicateReturn: "`{{name}}` starts with `{{word}}` but does not return a boolean.",
      procedureVerb:
        "`procedure` `{{name}}` must start with an allowed verb (eslint/settings/verbs.ts), not `{{word}}`.",
      vagueName:
        "`{{kind}}` function `{{name}}` starts with vague `{{word}}`: name the concept itself.",
    },
    schema: [
      {
        additionalProperties: false,
        properties: {
          formatPattern: { type: "string" },
          predicatePrefixes: { items: { type: "string" }, type: "array" },
          procedureVerbs: { items: { type: "string" }, type: "array" },
          vaguePrefixes: { items: { type: "string" }, type: "array" },
        },
        required: ["formatPattern", "predicatePrefixes", "procedureVerbs", "vaguePrefixes"],
        type: "object",
      },
    ],
    type: "suggestion",
  },
  name: "kind-naming",
});

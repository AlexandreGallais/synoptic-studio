import type { CssDeclaration } from "./css-declaration";

/**
 * The value of a `style` attribute: a declaration list, each declaration its property, a colon
 * and its value, separated by semicolons (`REF-CSS-STYLE-ATTR`, §3); none gives an empty value.
 *
 * @kind format
 * @param declarations - the declarations, in order
 * @returns `property: value; …`
 * @see REF-CSS-STYLE-ATTR
 */
export function formatCssDeclarations(declarations: readonly CssDeclaration[]): string {
  return declarations.map(([property, value]) => `${property}: ${value}`).join("; ");
}

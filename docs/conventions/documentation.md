# Code documentation (TSDoc)

A reader, the agent included, reads a function's block instead of its body, and the API pages are built from these blocks. So every block says what the code gives and where its formula comes from, and nothing else ([writing](./writing.md#api-reference-tsdoc)).

## Where (`jsdoc/require-jsdoc`)

Every top-level function, type and module constant carries a `/** … */` block. In `src/`, every type member does too (`/** Abscissa, growing to the right. */`), because the API pages' generator (`tools/api-pages`) fails the docs build on an undocumented property.

```ts
/**
 * Distance from a corner vertex to the tangent points of its fillet arc.
 *
 * Formula: d = r / tan(θ / 2), θ = interior angle of the corner.
 *
 * @kind geometry
 * @param radius - fillet radius, >= 0
 * @param interiorAngle - interior angle in radians, in ]0, π[
 * @returns setback distance along each adjacent edge
 * @see DERIV-fillet-setback
 */
```

## Shape (`jsdoc/match-description`, `jsdoc/sort-tags`, `jsdoc/tag-lines`)

- Write the description in sentences, with a capital letter and a final period. When there is a formula, write it in the description.
- Write `@param name - …` and `@returns …` as fragments without a final period. A helper of a test file (`*.test.ts`) may skip them: its typed signature sits under its description, and only the test beside it reads it (EN-093). A tag written there is still checked.
- Order the tags: `@kind`, then `@template @param @returns @yields @throws @rejects`, then `@see @deprecated`. Leave one blank line between the description and the tags, and none between tags.
- Write no type in JSDoc (`jsdoc/no-types`): the signature holds it, and a second copy drifts.
- Keep a block on one line when it fits: `/** Separator between commands. */`.
- Never write `@todo`, `@author`, `@version` nor `@since`: history is in Git, and open work is in the backlog.

## `@see` references

Checked by `local/see-references`.

Every `math`, `geometry` and `domain` function cites at least one source it was written from, so that its formula or rule can be checked against that source; a `format` or `procedure` function, such as the playground's clicks and displays, may cite one but need not (Product Owner, 2026-10-25).

| Form | Resolution | Example |
| --- | --- | --- |
| `REF-…` | a row of the records' `sources`, verified (`knowledge show REF-…`) | `@see REF-SVG2-PATHS` |
| `DERIV-name` | a row of the records' `derivations` | `@see DERIV-local-radius-clamp` |
| `RULE-page-anchor` | a part of a domain page, a document of the records and its rules in `rules` | `@see RULE-shapes-2-corner-radius` |
| `ADR-NNNN` | a row of the records' `decisions` | `@see ADR-0007` |
| backlog id | a row of the records' `stories`, `features` or `epics` | `@see US-010` |
| `docs/…/file.md#anchor` | an existing file | `@see docs/tooling/knowledge.md` |

- The CI reads each id by its form alone (`local/see-references`), having no records; `knowledge check-ids` checks each against the records before a pull request into `develop`. A source never read is never cited, and an unverified one never by `@see`.
- A non-trivial `math` or `geometry` function cites an external source (`REF-*`) or a derivation (`DERIV-*`), not only the domain's rule.
- When no source can be verified, write the derivation into the records or open a research request (`knowledge show PAGE-research`).

## API reference site

`tools/api-pages` turns each exported symbol's TSDoc into a page of the documentation site (`npm run docs:dev`, [documentation site](../tooling/documentation-site.md)). Every block tag but `@param`, `@returns` and `@template` becomes a section of the page, after the return value: `@kind` among them.

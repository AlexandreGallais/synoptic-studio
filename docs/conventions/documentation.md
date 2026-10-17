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

## `@see` references (`local/see-references`)

Every function cites at least one source it was written from, so that its formula can be checked against that source.

| Form | Resolution | Example |
| --- | --- | --- |
| `REF-…` | a row of `docs/references.md` **not marked `[unverified]`** | `@see REF-SVG2-PATHS` |
| `DERIV-name` | file `docs/derivations/name.md` | `@see DERIV-local-radius-clamp` |
| `ADR-NNNN` | file `docs/adr/NNNN-*.md` | `@see ADR-0007` |
| `docs/…/file.md#anchor` | an existing file | `@see docs/domain/shapes.md#_2-corner-radius` |

- A target that does not exist or is `[unverified]` is a lint error. A source never read is never cited.
- A non-trivial `math` or `geometry` function cites an external source (`REF-*`) or a derivation (`DERIV-*`), not only the domain page.
- When no source can be verified, write the derivation in `docs/derivations/` or open a research request ([research protocol](../research/)).

## API reference site

`tools/api-pages` turns each exported symbol's TSDoc into a page of the documentation site (`npm run docs:dev`, [documentation site](../tooling/documentation-site.md)). Every block tag but `@param`, `@returns` and `@template` becomes a section of the page, after the return value: `@kind` among them.

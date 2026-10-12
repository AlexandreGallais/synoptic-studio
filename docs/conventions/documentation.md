# Code documentation (TSDoc)

## Mandatory everywhere (`jsdoc/require-jsdoc`)

Every top-level function, type and module constant carries a `/** … */` block; in `src/`, every **type member** too (`/** Abscissa, growing to the right. */`), since TypeDoc fails the docs build on an undocumented property.

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

- Main description: **sentences** (capital letter, final period). The formula is written in it when one exists.
- `@param name - …` and `@returns …`: **fragments** without final period.
- Tag order: `@kind`, then `@template @param @returns @yields @throws @rejects`, then `@see @deprecated`. One blank line between the description and the tags, none between tags.
- No type in JSDoc (`jsdoc/no-types`): TypeScript holds them.
- One-line block when it fits on one line: `/** Separator between commands. */`.
- Forbidden: `@todo`, `@author`, `@version`, `@since` (history is in git, ongoing work in `docs/`).

## `@see` references (`local/see-references`)

**Every function** cites at least one source. Accepted targets:

| Form                    | Resolution                                                | Example                                       |
| ----------------------- | --------------------------------------------------------- | --------------------------------------------- |
| `REF-…`                 | row of `docs/references.md` **not marked `[unverified]`** | `@see REF-SVG2-PATHS`                         |
| `DERIV-name`            | file `docs/derivations/name.md`                           | `@see DERIV-local-radius-clamp`               |
| `ADR-NNNN`              | file `docs/adr/NNNN-*.md`                                 | `@see ADR-0007`                               |
| `docs/…/file.md#anchor` | existing file                                             | `@see docs/domain/shapes.md#_2-corner-radius` |

- A non-existent or `[unverified]` reference is a **lint error**. A source never consulted is never cited.
- Non-trivial `math` / `geometry`: an external source (`REF-*`) or a derivation (`DERIV-*`) — not a mere pointer to the domain.
- No verifiable source: write the derivation in `docs/derivations/`, or open a research request ([research protocol](../research/)).

## API reference site

TypeDoc turns this TSDoc into one page per exported symbol on the documentation site (`npm run docs:dev`, [documentation-site.md](../tooling/documentation-site.md)). The `@kind` tag is declared in `tsdoc.json`.

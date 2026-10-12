# Note 0001 — Tooling practices

**Dates**: 2026-10-07 and 2026-10-08 — research done by Claude Code (WebSearch / WebFetch), without a prior request.
**Resulting decisions**: ADR-0009 to ADR-0016.

## 1. Annotating the kind of a function

- TypeScript decorators attach only to class declarations, methods, accessors, properties and parameters; never to a free function (`REF-TS-DECORATORS`). The TC39 proposal (stage 3) also targets classes and their elements (`REF-TC39-DECORATORS`).
- Consequence: the function marker the Product Owner asked for can only be a comment tag, readable by ESLint → `@kind` (ADR-0011).

## 2. Function size and complexity

- Fowler: length matters less than the gap between intention and implementation; as soon as a fragment takes effort to read, extract it and name it after its "what". He finds any function longer than half a dozen lines suspicious (`REF-FOWLER-FUNCTION-LENGTH`).
- Beck, _Composed Method_: one method = one identifiable task, all its operations at the same level of abstraction; this naturally yields many methods a few lines long (`REF-BECK-SBPP`, content checked through secondary sources).
- McCabe / NIST: cyclomatic complexity counts independent paths; recommended threshold 10, tolerated up to 15 in some cases (`REF-NIST-500-235`).
- Retained synthesis: bound **complexity** strictly everywhere, and **length** by kind; a long procedure is acceptable when it is linear (a sequence of named steps).

## 3. ESLint and Prettier

- Prettier recommends `eslint-config-prettier` (turns off conflicting rules) and advises against plugins running Prettier inside the linter (slow, editor noise) (`REF-PRETTIER-LINTERS`).
- Prettier reads `.editorconfig`: `end_of_line`, `indent_style`, `indent_size` / `tab_width`, `max_line_length` (`REF-PRETTIER-CONFIG`).
- ESLint loads `eslint.config.ts` through `jiti` ≥ 2.2.0, or natively behind an experimental flag (`REF-ESLINT-TS-CONFIG`).

## 4. Barrel files

- Costs reported at scale: slower builds, degraded tree-shaking, import cycles (`REF-ATLASSIAN-BARRELS`, `REF-TKDODO-BARRELS` — level-6 sources, context only).
- Mitigation: small dependency-free library, `sideEffects: false`, cycles forbidden, ancestor barrel imports forbidden (ADR-0012).

## 5. One export per file

- Angular: focus a file on a single concept, match the file name to the identifier inside (`REF-ANGULAR-STYLE`).
- Airbnb 23.6–23.7: the file name exactly matches the exported name; camelCase for a function (`REF-AIRBNB-STYLE`).
- Google: named exports only, minimal exported surface (`REF-GOOGLE-TS-STYLE`).
- Retained: one value per file, named exactly like it; a type stays next to the value only if it is part of its signature (ADR-0015).

## 6. Default parameters

- Google: allowed, "use sparingly", initializers without side effects. Airbnb: use the syntax rather than mutating arguments, keep defaults last (`REF-GOOGLE-TS-STYLE`, `REF-AIRBNB-STYLE`).
- Neither guide addresses a parametric model where defaults are business data. Retained: no default nor optional parameter in `src/`; defaults belong to the model (ADR-0016).

## 7. Documentation site

- VitePress is the Vite-native static site generator, used by the Vite and Vitest docs; stable 1.6.4 bundles Vite 5 (vulnerable esbuild per `npm audit`), 2.0 alpha uses Vite 8 (`REF-VITEPRESS`).
- TypeDoc + `typedoc-plugin-markdown` + `typedoc-vitepress-theme` generate one Markdown page per exported symbol with a VitePress sidebar (`REF-TYPEDOC-MARKDOWN`).
- PDF: neither Docusaurus nor VitePress exports PDF natively; both rely on community tools (`vitepress-export-pdf`, last published 2023, beta; Docusaurus has comparable third-party tools). Not a reason to prefer Docusaurus (React, separate toolchain). PDF export is left open (see [documentation-site.md](../../tooling/documentation-site.md)).

## 8. Ecosystem on 2026-10-08

- TypeScript 7.0 is out; `typescript-eslint` 8.71 requires `typescript <6.1` → TypeScript 6.0 pinned (ADR-0009). TypeDoc 0.28 supports TypeScript 6.0.
- ESLint 10, `eslint-plugin-unicorn` 77 (~370 rules), `eslint-plugin-import-x` 4, Vite 8, Vitest 5, VitePress 2.0 alpha.

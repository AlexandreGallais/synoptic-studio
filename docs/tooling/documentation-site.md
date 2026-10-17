# Documentation site

## Stack

- **VitePress** (`REF-VITEPRESS`): the `docs/` folder is the site. `npm run docs:dev` serves it, `npm run docs:build` builds it in `docs/.vitepress/dist/`.
- **Diagrams**: a `mermaid` code block is drawn in the browser by the site's theme (`docs/.vitepress/theme/`), loaded with the page; GitHub draws the same block natively (`REF-MERMAID`, `REF-GH-DIAGRAMS`).
- **API pages**: `npm run docs:api` runs `tools/api-pages`, a Rust crate of the workspace on Oxc's parser (EN-072, note 0023), which writes one Markdown page per exported function, type alias and constant of `src/index.ts`, reached through its barrels, into `docs/api/` (not committed), with `index.md` and the sidebar file `api-sidebar.json` that `docs/.vitepress/config.ts` reads. Each page shows the signature as written in the annotations, `Defined in` with its file and line, the TSDoc summary, then its type parameters, parameters, return value, properties and remaining block tags (`@kind`, `@see`) as sections. Types link to their pages, so one navigates from function to function. `npm run docs:api` takes about 0.35 s once the crate is built (0.1 s for the generator itself), against 3.5 s for TypeDoc.
- **Its checks** fail the docs build before any page is written, each case named with its file and line, as TypeDoc's validation did with warnings as errors: an export or a property of its object types without a TSDoc block (a type's arguments and a function type's parameters read too, AUD-021), an `@param` naming no parameter, a `{@link}` naming nothing exported, a name of the library's own reachable from a public signature but not exported, and an annotation it cannot read (a constant's type it cannot read from its value: write the annotation).
- **The same pages as TypeDoc's**: the Markdown, the index and the sidebar are those `typedoc-plugin-markdown` and `typedoc-vitepress-theme` wrote (`REF-TYPEDOC-MARKDOWN`), compared byte for byte on 2026-10-17: 527 of the 537 pages and the sidebar identical. The 10 others show the type as written where TypeDoc showed the compiler's resolved type: an alias read from an indexed access (`Alignment`, `HandleWay`, `HandleMode`) linked instead of written out as its union, a union's members in their written order, no default type argument added (`DataView`, not `DataView<ArrayBufferLike>`), and the parentheses of an array of an intersection kept. A type alias the module declares without exporting it is written out, as the compiler showed it.

## Playground

The playground is built by `npm run playground:build` (`vite build --mode playground`) into `docs/.vitepress/dist/playground/` and published with the site: it shows the state of `main` at the last release. `check:all` builds it too when the branch changed the site, the library or the playground (EN-085, [commands](./commands.md)), so a broken playground fails the pre-push hook and the CI into `main`.

## Layout

| Folder | Section of the site |
| --- | --- |
| `docs/index.md` | home |
| `docs/domain/` | Domain |
| `docs/adr/` | Decisions |
| `docs/conventions/` | Conventions |
| `docs/tooling/` | Tooling |
| `docs/derivations/` | Derivations |
| `docs/research/` | Research |
| `docs/references.md` | References |
| `docs/api/` | API (generated) |

Sidebar entries are built from the Markdown files of each folder (title = first `#` heading); `README.md` files are served as the folder's index. Dead links fail the build.

## Writing pages

- English, Markdown, one `#` title per page.
- Link with relative Markdown links (`[ADR-0011](../adr/0011-function-kinds.md)`) so that VitePress checks them.
- Never write double curly braces in a page, even in inline code: VitePress evaluates them as a Vue expression. Use a fenced code block or wrap the text in `<span v-pre>`.

## PDF

Neither VitePress nor Docusaurus exports PDF natively; both rely on community tools (`vitepress-export-pdf`, last published in 2023 as a beta). This is not a reason to switch to Docusaurus. PDF export is **not set up yet**: when needed, the options are a maintained VitePress PDF plugin, or printing the built site with a headless browser. Decision to take then, through an ADR.

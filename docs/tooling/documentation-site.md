# Documentation site

## Stack

- **VitePress** (`REF-VITEPRESS`): the `docs/` folder is the site; `npm run docs:dev` serves it, `npm run docs:build` builds it into `docs/.vitepress/dist/`. A dead link fails the build.
- **Diagrams**: a `mermaid` code block, drawn in the browser by the site's theme (`docs/.vitepress/theme/`) and natively by GitHub (`REF-MERMAID`, `REF-GH-DIAGRAMS`).
- **API pages**: `npm run docs:api` runs `tools/api-pages` (Oxc's parser), which writes one page per exported function, type alias and constant of `src/index.ts` into `docs/api/` (not committed), with its index and the sidebar file `api-sidebar.json`. Each page shows the signature as written, its file and line, the TSDoc summary, its parameters, return value, properties and block tags (`@kind`, `@see`); types link to their pages. The pages are those TypeDoc's Markdown plugin wrote (`REF-TYPEDOC-MARKDOWN`), except that a type is shown as written rather than as the compiler resolves it.
- **API checks**: the generator fails the docs build, naming file and line, on an export or a property without a TSDoc block, an `@param` naming no parameter, a `{@link}` naming nothing exported, a name of the library reachable from a public signature but not exported, and an annotation it cannot read (write the constant's type). The public API is documented or the build stops.

## Playground

`npm run playground:build` builds the playground into `docs/.vitepress/dist/playground/`, published with the site: it shows `main` at the last release. `check:all` builds it whenever the site is built ([lanes](./commands.md#the-lanes-of-the-full-check)), so a broken playground fails the push.

## Layout

The sidebar and the navigation bar follow one order, `GROUPS` in `docs/.vitepress/config.ts`, ordered for a person: what the editor is, how the project works and where it stands, the order of the epics, then the API and the commands. Everything else the agent reads in its records (`target/release/knowledge show|search`).

| Group | Sections, in order (pages of `docs/`) |
| --- | --- |
| The project | Overview (`the-editor.md`, `how-it-works.md`, `where-it-stands.md`), Epics (`backlog/`) |
| Reference | API (`api/`, generated), Tools (`commands.md`, `tooling/knowledge.md`) |

A section lists its folder's Markdown files, titled by their first `#` heading, a `README.md` as its index and the templates (`_*.md`) left out, or the pages it names; the story files are reached from the backlog's indexes, not the sidebar. `docs/index.md` is the home page, its links in the same order.

## Writing pages

- Link with relative Markdown links (`ADR-0011`), so that VitePress checks them.
- Never write double curly braces, even in inline code: VitePress evaluates them as a Vue expression. Use a fenced code block or `<span v-pre>`.

## PDF

Not set up: neither VitePress nor Docusaurus exports PDF natively. When it is needed, an ADR chooses between a maintained VitePress PDF plugin and printing the built site with a headless browser.

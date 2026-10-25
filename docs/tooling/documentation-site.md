# Documentation site

## Stack

- **VitePress** (`REF-VITEPRESS`): the `docs/` folder is the site; `npm run docs:dev` serves it, `npm run docs:build` builds it into `docs/.vitepress/dist/`. A dead link fails the build.
- **Diagrams**: a `mermaid` code block, drawn in the browser by the site's theme (`docs/.vitepress/theme/`) and natively by GitHub (`REF-MERMAID`, `REF-GH-DIAGRAMS`).
- **API pages**: `npm run docs:api` runs `tools/api-pages` (Oxc's parser), which writes one page per exported function, type alias and constant of `src/index.ts` into `docs/api/` (not committed), with its index and the sidebar file `api-sidebar.json`. Each page shows the signature as written, its file and line, the TSDoc summary, its parameters, return value, properties and block tags (`@kind`, `@see`); types link to their pages. The pages are those TypeDoc's Markdown plugin wrote (`REF-TYPEDOC-MARKDOWN`), except that a type is shown as written rather than as the compiler resolves it.
- **API checks**: the generator fails the docs build, naming file and line, on an export or a property without a TSDoc block, an `@param` naming no parameter, a `{@link}` naming nothing exported, a name of the library reachable from a public signature but not exported, and an annotation it cannot read (write the constant's type). The public API is documented or the build stops.

## Playground

`npm run playground:build` builds the playground into `docs/.vitepress/dist/playground/`, published with the site: it shows `main` at the last release. `check:all` builds it whenever the site is built ([lanes](./commands.md#the-lanes-of-the-full-check)), so a broken playground fails the push.

## Layout

The sidebar and the navigation bar follow one order, `GROUPS` in `docs/.vitepress/config.ts`, ordered for a reader, the agent first (REV-009): what the project is, its domain and its library's API, how to work, the tooling, then the records, grouped and last.

| Group | Sections, in order (folders of `docs/`) |
| --- | --- |
| The project | Domain (`domain/`), API (`api/`, generated) |
| How to work | Conventions (`conventions/`), Process (`process/`, its index and `agile.md`), Backlog (`backlog/`), Epics (`backlog/epics/`), Features (`backlog/features/`), Research (`research/`, the protocol), Playbook (`playbook/`) |
| Tooling | Tooling (`tooling/`) |
| Records | Decisions (`adr/`), Research notes (`research/notes/`), Research requests (`research/requests/`), Derivations (`derivations/`), References (`references.md`), Guide (`guide/`, the pages of past features and epics), Retrospectives (`process/retrospectives/`), Logs (the journal, the improvement and feedback logs, the tool incidents and the history of `process/`) |

A section lists its folder's Markdown files, titled by their first `#` heading, a `README.md` as its index and the templates (`_*.md`) left out, or the pages it names; the story files are reached from the backlog's indexes, not the sidebar. `docs/index.md` is the home page, its links in the same order.

## Writing pages

- Link with relative Markdown links (`[ADR-0011](../adr/0011-function-kinds.md)`), so that VitePress checks them.
- Never write double curly braces, even in inline code: VitePress evaluates them as a Vue expression. Use a fenced code block or `<span v-pre>`.

## PDF

Not set up: neither VitePress nor Docusaurus exports PDF natively. When it is needed, an ADR chooses between a maintained VitePress PDF plugin and printing the built site with a headless browser.

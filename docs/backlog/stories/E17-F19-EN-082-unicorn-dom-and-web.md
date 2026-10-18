---
id: EN-082
epic: E17
feature: F19
title: Unicorn's DOM and web rules in Rust
status: done
points: 5
---

# E17 · F19 · EN-082 — Unicorn's DOM and web rules in Rust

Rewrite in the project's Rust linter the 31 kept rules of `eslint-plugin-unicorn` on the DOM and the web, which oxlint runs only as JS plugins; kept on purpose since the playground uses the DOM, files and `URL` (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 31 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-082.Tn`.

- [ ] T1 — DOM, first half (10): `better-dom-traversing`, `dom-node-dataset`, `no-canvas-to-image`, `no-incorrect-query-selector`, `no-invalid-dom-token`, `no-invalid-style-set-property`, `no-late-current-target-access`, `no-late-event-control`, `no-prevent-default-in-passive-listener`, `no-selector-as-dom-name`, each with its documented cases as tests (3 h)
- [ ] T2 — DOM, second half (9): `no-unsafe-dom-html`, `prefer-add-event-listener-options`, `prefer-dom-node-html-methods`, `prefer-dom-node-replace-children`, `prefer-observer-apis`, `prefer-path2d`, `prefer-scoped-selector`, `prefer-toggle-attribute`, `require-passive-events`, each with its documented cases as tests (3 h)
- [ ] T3 — Web (12): `no-blob-to-file`, `no-conflicting-constraints`, `no-ineffective-csp-directives`, `no-invalid-boolean-attribute-value`, `no-invalid-file-input-accept`, `no-invalid-url-protocol-comparison`, `no-url-in-search-params`, `prefer-location-assign`, `prefer-url-can-parse`, `prefer-url-href`, `prefer-url-search-parameters`, `require-css-escape`, each with its documented cases as tests (3 h)
- [ ] T4 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)

## Timings (ADR-0069)

Measured on this Mac, 2026-10-17, on `npm run lint`'s targets (2,053 files):

| Tool | Before | After |
| --- | --- | --- |
| Rust linter (`tools/lint`) | 0.12–0.13 s | 0.13 s, 25 rules more |
| ESLint | 13.1 s, the 31 rules 437 ms (`TIMING=all`) | 12.4 s, 31 rules fewer |

## For the main thread

- Rules: of the 31, 25 run in the project's linter (`tools/lint/src/rules/unicorn/dom/`, 16, and `web/`, 9), each with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests; the 16 DOM rules and `no-invalid-boolean-attribute-value`, `prefer-location-assign` and `require-css-escape` only where a DOM exists (`scopes::DOM_FILES`: `src/render/`, `src/interaction/`, `playground/`, `docs/.vitepress/theme/`), the six URL and `Blob` rules on every file. `dom-node-dataset` runs in oxlint as `unicorn/prefer-dom-node-dataset`, its former name (same seven findings on the probe; oxlint also autofixes the reads ESLint only suggests). Dropped with their reasons in note 0023 and `DROPPED_RULES`: `no-conflicting-constraints`, `no-ineffective-csp-directives` and `no-invalid-file-input-accept` (they read HTML, CSS or JSX only, which nothing here lints), `no-canvas-to-image` and `prefer-path2d` (the code draws no canvas; its one canvas measures text). The 31 are off in ESLint.
- Compared: on the tree, none from either linter; on 24 probe files of the documented and planted cases, the same findings and the same fixed text, ESLint given the browser globals (`document`, `Document`, `window`…) it is not configured with here: without them, its global trackers report no `document.write()` nor `Document.parseHTMLUnsafe()`, which the Rust rule reports as the documentation does.
- Shared helpers: `rules::unicorn::annotations` reads what a written type says without the compiler (the plugin's `utils/type-helpers.js`), `web/urls.rs` tells a `URL` (`utils/is-url.js`), `dom/events.rs` the event handlers and their suspensions (`shared/late-event-handler.js`), `dom/css_properties.rs` holds `@eslint/css-tree` 4.1.1's 722 property names for `no-invalid-style-set-property`; `expressions::is_written` (`utils/is-left-hand-side.js`).
- Note 0023's counts were applied after the rebase on EN-080 and EN-081: Unicorn 148 kept and 49 dropped, all 199 and 66, the drops' heading ending with EN-082's five.
- Rule-docs: none affected, the story moves lint rules between linters and changes no rule of `docs/domain/`.

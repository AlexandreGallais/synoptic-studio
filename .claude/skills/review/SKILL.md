---
name: review
description: Run the review held at each return of the Product Owner (REV story) — the features finished since the last merge into `main`, the naming pass over the code changed since, the tool cards, the method, the docs. Use when the Product Owner comes back, or for a story of type REV. Also use for a feature's demo page.
argument-hint: "[REV id, or a feature id for its demo]"
---

# Review (and feature demo)

Target: $ARGUMENTS. Decisions: ADR-0023, ADR-0086. Writing rules: `docs/conventions/writing.md`, and `docs/guide/README.md` for guide pages.

## Feature demo

1. Write `docs/guide/<feature-slug>.md` from `docs/guide/_feature-demo.md`: what the user can do, the ideas with every term explained, pictures produced by the library, the checklist's address and its lines with what each shows, limits.
2. Put the validation **in the playground** as lines of the epic's checklist (`playground/checklists/`, opened by `?checklist=<epic>`, US-051): one line per option kept and one per decision to confirm, each what to try and what to see in one short sentence, a check box before it; no guided steps, no Previous nor Next; test that the list names every option kept and none removed. Pictures of the demo page are drawn by the library (SVG files in `docs/guide/images/`); flows and structures are Mermaid blocks.
3. List the page in `docs/guide/README.md`; open the pull request; ask the Product Owner to try the checklist's lines, in any order and over several sessions if they wish, and tell you what they see; write the result and each remark in neutral English, never their words (`docs/conventions/writing.md`): the **Validation** table of the page, remark by remark, with what was done or where it went. Read their messages for intent.
4. Feedback that reveals a misunderstanding: record it, propose stories or domain changes, never adjust silently.
5. Compare the Product Owner's account (often spoken) with what each test card and the demo meant to show: did they understand what you wanted to show? Record each gap as a misunderstood need (backlog) or an unclear demo (improve the cards and this skill).
6. Then the feature pull request into `develop` (`/run`, A feature's end), the pull request `develop` → `main` at the next review (`/run`, Review); the next feature starts from `develop` with the stories its epic's refinement counted, a short spike only for a question nobody foresaw (`/spike`, ADR-0100).

## Review at a return (REV)

No epic has a closing (ADR-0086): a review is held at each return of the Product Owner, once or twice a day, a feature under way finished first. Its `REV` story (`docs/backlog/_review.md`), its branch from `develop`, covers everything since the last merge into `main`.

1. **The features finished since the last merge into `main`**: their texts for the release and their demo pages, re-read; the checklist lines they left re-run on the current `develop` (`?checklist=<epic>`), a regression being a finding.
2. **The naming pass over the code changed since then** (ADR-0049), at every review whatever its time box, never left to the next one: a feature's audit sees one feature, the pass sees them together. Names only, behaviour unchanged: each function, type, property, argument, constant and file of the code changed, against each other and the rest of the project (one word per concept, one name per argument, one verb pattern per kind of function), from an inventory built mechanically, by one worker (REV-008). Every calculation a field names is a call to a function of that name with its `@see`; the golden results written before it compared after it, bit for bit (`npm run golden:check`).
3. **The method**, with the Product Owner: the tool incident cards, the logs, what helped and hurt (`/retro`); the cards fixed in the review or decided, the improvements decided written as ADRs, rules or stories.
4. Prepare questions for the Product Owner: anything built that they did not mean? the next feature still the right one? the epics' division by theme still right?
5. Record the answers in the `REV` story; turn changes into backlog items and domain updates (Product Owner decides priority); bring the docs up to date.
6. Explain to the Product Owner in French, in plain words, in the summary message; offer to walk through the playground together. Then `develop` → `main` (`/run`, Review).

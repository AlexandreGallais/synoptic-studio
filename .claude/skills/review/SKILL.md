---
name: review
description: Run the epic review (REV story) — plain-language epic report, the epic's checklist, Product Owner feedback, backlog adjustments. Use when all features of an epic are validated, or for a story of type REV. Also use for the demo page of a feature validation (VAL).
argument-hint: "[REV id, or VAL id for a feature demo]"
---

# Epic review (and feature demo)

Target: $ARGUMENTS. Decision: ADR-0023. Writing rules: `docs/conventions/writing.md`, and `docs/guide/README.md` for guide pages.

## Feature demo (VAL)

1. Write `docs/guide/<feature-slug>.md` from `docs/guide/_feature-demo.md`: what the user can do, the ideas with every term explained, pictures produced by the library, the checklist's address and its lines with what each shows, limits.
2. Put the validation **in the playground** as lines of the epic's checklist (`playground/checklists/`, opened by `?checklist=<epic>`, US-051): one line per option kept and one per decision to confirm, each what to try and what to see in one short sentence, a check box before it; no guided steps, no Previous nor Next; test that the list names every option kept and none removed. Pictures of the demo page are drawn by the library (SVG files in `docs/guide/images/`); flows and structures are Mermaid blocks.
3. List the page in `docs/guide/README.md`; open the pull request; ask the Product Owner to try the checklist's lines, in any order and over several sessions if they wish, and tell you what they see; write the result and each remark in neutral English, never their words (`docs/conventions/writing.md`): the **Validation** table of the page, remark by remark, with what was done or where it went. Read their messages for intent.
4. Feedback that reveals a misunderstanding: record it, propose stories or domain changes, never adjust silently.
5. Compare the Product Owner's account (often spoken) with what each test card and the demo meant to show: did they understand what you wanted to show? Record each gap as a misunderstood need (backlog) or an unclear demo (improve the cards and this skill).
6. Then the `VAL`'s pull request into `develop` (`/run`, Validation), the pull request `develop` → `main` at the Product Owner's review (`/run`, Review), and the next feature prepared with the Product Owner: interview, research spike.

## Epic review (REV)

1. Write `docs/guide/<epic-slug>.md` from `docs/guide/_epic-report.md`: one paragraph summary, feature by feature with links to their demo pages, terms, a user test crossing all features, decisions in plain words, limits.
2. Re-run the epic's whole checklist on the current `develop` (`?checklist=<epic>` in the playground): a regression is a finding. The Product Owner reruns it too, as the epic's full recheck; the guided steps written up to E15 stay in `?guide=epic` for a closer look.
3. Prepare questions for the Product Owner: closure criteria of the epic met? anything built that they did not mean? next epic still the right one?
4. Record the answers in the report's feedback section; turn changes into backlog items and domain updates (Product Owner decides priority).
5. Explain to the Product Owner in French, in plain words, in the summary message; offer to walk through the playground together.

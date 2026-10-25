---
name: review
description: Run the review held at each return of the Product Owner (REV story) — the features finished since the last merge into `main`, the test cards of their user stories, the naming pass over the code changed since, the tool cards, the method, the docs. Use when the Product Owner comes back, or for a story of type REV.
argument-hint: "[REV id]"
---

# Review

Target: $ARGUMENTS. Decisions: ADR-0023, ADR-0086, ADR-0100. Writing rules: `docs/conventions/writing.md`.

No epic has a closing (ADR-0086) and no feature a demo page, a checklist nor a `VAL` of its own (ADR-0100): a review is held at each return of the Product Owner, once or twice a day, a feature under way finished first. Its `REV` story (`docs/backlog/_review.md`), its branch from `develop`, covers everything since the last merge into `main`.

1. **The features finished since the last merge into `main`**: their texts for the release, re-read; the **test cards** (`## Product Owner test`) of the `US` stories merged since then, listed for the Product Owner, who tries on `develop` what changed, only that, in any order (ADR-0100, decision 4). Write the result and each remark in neutral English, never their words, in the `REV` story, with what was done or where it went; read their messages for intent.
2. **What they understood**: compare their account (often spoken) with what each test card meant to show. A gap is a misunderstood need (a story or a domain change, proposed, never adjusted silently) or an unclear card (improve the cards and the `story` skill).
3. **The naming pass over the code changed since then** (ADR-0049), at every review whatever its time box, never left to the next one: a feature's audit sees one feature, the pass sees them together. Names only, behaviour unchanged: each function, type, property, argument, constant and file of the code changed, against each other and the rest of the project (one word per concept, one name per argument, one verb pattern per kind of function), from an inventory built mechanically, by one worker (REV-008). Every calculation a field names is a call to a function of that name with its `@see`; the golden results written before it compared after it, bit for bit (`npm run golden:check`).
4. **The method**, with the Product Owner: the tool incident cards, the logs, what helped and hurt (`/retro`); the cards fixed in the review or decided, the improvements decided written as ADRs, rules or stories.
5. Prepare questions for the Product Owner: anything built that they did not mean? the next feature still the right one? the epics' division by theme still right?
6. Record the answers in the `REV` story; turn changes into backlog items and domain updates (Product Owner decides priority); bring the docs up to date.
7. Explain to the Product Owner in French, in plain words, in the summary message; offer to walk through the playground together. Then `develop` → `main` (`/run`, Review).

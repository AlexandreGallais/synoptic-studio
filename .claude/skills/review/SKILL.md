---
name: review
description: Run a review (REV story) when the Product Owner asks for one — the features finished since the last merge into `main`, the test cards of their user stories, the naming pass over the code changed since, the tool cards, the dependency updates, the method, the person's docs. Use only when the Product Owner asks for a review, or for a story of type REV.
argument-hint: "[REV id]"
---

# Review

Target: $ARGUMENTS. A review is held only when the Product Owner asks for one, a feature under way finished first. Its `REV` story, its branch from `develop`, covers everything since the last merge into `main`.

1. **The features finished since the last merge into `main`**: their texts for the release, re-read; the **test cards** of the `US` stories merged since then (`knowledge sql "SELECT id, test_card FROM stories …"`), listed for the Product Owner, who tries on `develop` what changed, only that. Their result and each remark go into the `REV`'s row in neutral English; the records may keep their words.
2. **What they understood**: compare their account with what each test card meant to show. A gap is a misunderstood need (a story or a rule change, proposed, never adjusted silently) or an unclear card (improve the cards and the `story` skill).
3. **The naming pass over the code changed since then**, at every review, never left to the next one: names only, behaviour unchanged, each function, type, property, argument, constant and file of the code changed, against each other and the rest of the project, from an inventory built mechanically, by one worker. Every calculation a field names is a call to a function of that name; the golden results compared after it, bit for bit (`npm run golden:check`).
4. **The dependency updates**: `npm run deps:outdated`, each update's release notes read, a version at least 7 days old, then updated on the `REV`'s branch.
5. **The method**, with the Product Owner: the tool incident cards, the improvement and feedback rows, what helped and hurt (`/retro`); the cards fixed in the review or decided, the improvements decided written as ADRs, rules or stories.
6. **The person's docs refreshed** from the records: `docs/the-editor.md`, `docs/how-it-works.md`, `docs/where-it-stands.md` (the epics done, kept true by `tests/person-pages.test.ts`), `README.md`, the order of the epics.
7. Prepare questions for the Product Owner: anything built that they did not mean? the next feature still the right one? the epics' division still right?
8. Explain to the Product Owner in French, in plain words; offer to walk through the playground together. Once they validate the review in the chat: the `REV` into `develop`, then `develop` → `main` merged by the agent, and a release when they ask (`/run`, Review).

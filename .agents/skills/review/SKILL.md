---
name: review
description: Run a review (REV story) when the Product Owner asks for one — what was built since the last merge into `main`, its test cards, the naming pass, the tool incidents, the dependency updates, the method and tools with the agent's list of needs — then, once validated in the chat, `develop` into `main` by its label, and the release on request. Use only when the Product Owner asks for a review, or for a story of type REV.
argument-hint: "[REV id]"
---

# Review

Target: $ARGUMENTS. Held only when the Product Owner asks, a feature under way finished first. Its `REV` story, a branch from `develop` (`target/release/worktree-new REV-<nnn>`), covers everything since the last merge into `main`.

1. **What was built**: the features since then, re-read; the test cards of their user stories (`knowledge sql "SELECT id, test_card FROM stories …"`) for the Product Owner to try on `develop`. Their remarks go into the `REV`'s row; a gap between their account and a card is a need misread (a story or a rule, proposed) or a card unclear.
2. **The naming pass** over the code changed since then, by one worker, names only, from an inventory built mechanically: each function, type, property, argument (every parameter's name too), constant and file, against each other and the project; every calculation a field names is a call to a function of that name; the golden check after it (`target/release/heavy-lock npx vitest run tests/golden-results.test.ts --mode golden-check`), bit for bit.
3. **The tool incidents** (`knowledge sql "SELECT … FROM incidents …"`): each fixed in the review, or decided.
4. **The dependency updates**, grouped: `target/release/dependencies outdated`, each update's release notes read, at least 7 days old.
5. **Method and tools**, facts first, every `improvements` row of kind `friction` or `need` presented unfiltered, with each wish to change a table, a tool or the method: `target/release/epic-check <epic> <day of the last merge into main>` per epic; `target/release/metrics --since <day>` (commands by count and time, the slowest, time and tokens per story); the improvement, feedback and incident rows. Then one table for the Product Owner, a row per point with a recommendation and what it changes for them, and the agent's own list of what it needs — tools, tables, settings, accesses — each asked as "I need X", their answer yes, no or later, with its reason.
6. **Decided** points become decisions, stories, rules or checks; the review's record in its `REV` story's items (`knowledge sql "INSERT INTO story_items …"`), the rows treated marked done; a repeated mistake gets a check, not a sentence, in `AGENTS.md`, `CLAUDE.md` or `.agents/`.
7. **No change in `docs/`**: the docs stay as they are until the Product Owner says otherwise; the open GitHub issues, if any, triaged (`/triage`).
8. **Explained in French**, in plain words, with the questions: anything built they did not mean? the next feature still the right one?
9. **Once they validate the review in the chat**: the `REV` into `develop` (`target/release/heavy-lock --lock merge target/release/story-finish develop …`); then `target/release/release notes` on an up-to-date `develop`, each `[draft]` line rewritten into a file, `target/release/release check <file>` until it passes, `gh pr create --base main --head develop --body-file <file>`, labeled `po-validated`: `main-automerge.yml` merges it once its checks pass (`gh pr checks <n>`; a failure: `gh run view <run> --log-failed`).
10. **The release**, only when the Product Owner asks for it, after the merge into `main`: `gh workflow run release.yml --ref main`; `release publish` takes the version from the tags and the notes from the pull requests' bodies. A body breaking a rule stops it: fix the body on GitHub, run it again.

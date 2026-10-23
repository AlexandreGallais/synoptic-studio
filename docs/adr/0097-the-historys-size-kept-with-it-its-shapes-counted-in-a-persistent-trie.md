# ADR-0097 — The history's size kept with it, the states holding each shape counted in a persistent trie

**Status**: Accepted (EN-147, 2026-10-23): the story left to the agent how the counts are kept without copying a map of every shape at each step; an exception to ADR-0014's "no mutation" for one memo, registered as ADR-0066 asks.

## Context

- The history's size (`historySize`, ADR-0095, decision 4) counts each shape a past or future state holds and the present does not, once, and every reference and index those states hold. AUD-042 measured a step at 63 ms with 80 000 moves of one rectangle once each text length was kept (ADR-0096), about the 80 380 the default 100 MB holds; the Product Owner asked for a history pleasant to use with up to about 1 000 symbols (SP-029's answer 6). ADR-0096, decision 3, left the size kept from step to step to F41.
- AUD-042's proposal: the history holds, beside its states, its size and, for each shape, how many past and future states hold it, each state counting a shape once; a state added or removed, and the present changed, update both from the shapes they move alone (EN-147).
- Those counts change at each step: a map of them copied at each step, so that the history before keeps its own (ADR-0014), copies every shape held. Measured in EN-147 on Node 24, a `Map` of 80 000 objects is copied in 4.5 to 6.5 ms; the designer may set up to 1 000 MB (US-076), ten times as many shapes, and smaller shapes hold more of them in the same size.
- A persistent structure gives "access to any version, old or new" (`REF-DRISCOLL-1989`) at the cost of copying, at a change, a node "and ancestors (back to root of data structure)", where a full copy is "bad" (`REF-MIT-6854-PERSISTENT`); collections built so make "'modified' versions, by utilizing structural sharing" (`REF-CLOJURE-DATA-STRUCTURES`, `REF-IMMUTABLE-JS`). ECMAScript gives an object no hash nor number by which a tree could place it: a `Map` or a `WeakMap` finds it by identity alone (ADR-0025, decision 1).

## Decision

1. **The size kept with the history.** `SizedHistory` is a history and its measure: `size`, equal to `historySize`'s, and `held`, how many past and future states hold each shape. A state added to the past or to those to come holds each of its shapes once more, one held by none before and not in the present adding its text's length times 2, and adds 8 bytes for each reference and index (`measureWithState`); a state removed, the reverse (`measureWithoutState`); the present changed, each shape it leaves that a state holds adds its text, each it takes takes it away (`measureWithPresent`). A step, an undo and a redo are those of `historyWithStep`, `undoneHistory` and `redoneHistory`, their measure updated from the states they move; the bound drops the oldest past states one by one while the size is over the size set and more than the steps kept are past (`sizedBoundedHistory`), the history `boundedHistory` gives.
2. **The counts in a persistent trie.** `HeldCounts` is a trie of 32 branches by an integer key of the shape, its digits in base 32 from the highest: a count set copies the nodes on its key's path and shares every other node with the counts before (`withHeldCount`, path copying), and a node left without a count is dropped. 80 000 keys take 4 levels: a step moving one shape copies 4 nodes of at most 32 slots for each shape it moves, whatever the history holds. The 32 branches are the agent's choice.
3. **One key for each shape, kept once given.** `shapeKey` gives each shape the next integer from 0 the first time it is asked, kept in a module's `WeakMap` by the shape, so that an entry goes with its shape; its writes are `lint-disable-next-line functional/immutable-data, functional/no-expression-statements` with their reason, registered in `docs/conventions/exceptions.md` (ADR-0066). Two shapes of equal texts are two objects and get two keys, as `historySize` counts them.
4. **The measure counted again stays the reference.** `historySize` and `boundedHistory` stay: a history measured from its states (`measuredHistory`) equals its kept size, the property of EN-147 checks the kept size against `historySize` after any sequence of steps, undos, redos and bounds, and a history bounded against `boundedHistory`'s.
5. **Where.** The counts and the keys in `src/model/held-shapes/`, with the bytes of a code unit and of a slot that `historySize` counts by; the history with its size kept in `src/model/sized-history/`; the page keeps a `SizedHistory` for each page and shows its `size`.

Example (A, a rectangle 100 × 50 at (10, 20), 614 characters; B at (200, 20)): A and B drawn, A nudged to (11, 20). The step adds the present A and B to the past, both in the present: A and B held by 1, 3 slots, 24 bytes; then the present leaves A, held by 1: 1 228 bytes, 1 252 in all, as `historySize` counts it.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| A persistent trie by the shape's key (chosen) | a step copies what it moves, a few nodes for each shape; every history keeps its own counts, the core's functions pure (ADR-0014); ECMAScript alone (ADR-0025) | the trie's code and tests; a key memo in the core, one more exception to the lint |
| A `Map` of the counts copied at each step | the simplest; no key, no exception | each step copies every shape held: 4.5 to 6.5 ms at 80 000, ten times as many at the 1 000 MB the designer may set |
| A `Map` kept by the shell and changed in place | no copy | an "ephemeral" structure (`REF-DRISCOLL-1989`): the history before loses its counts, so that the core's functions return what is valid only for the last history; the core no longer plain data and pure functions (ADR-0014, ADR-0025) |
| The size counted again at each step (ADR-0096 alone) | nothing more | 63 ms a step at 80 000 moves, AUD-042's measure |

## Consequences

- The page records with `sizedHistoryWithStep`, `sizedUndoneHistory`, `sizedRedoneHistory` and `sizedBoundedHistory`, starts its history from `measuredHistory` and shows `size`; it no longer keeps the bound each history was bounded by nor the size shown, a history within its size bounded by one comparison.
- `interaction.md` §7 (Its bound) says how the page keeps the size.
- A transcription to another language (ADR-0025) keeps the trie or uses that language's persistent map; the keys stand for an object's identity, which some languages give as a hash.
- The measures of a step before and after are in EN-147's story file.

## References

ADR-0014, ADR-0025, ADR-0066, ADR-0095, ADR-0096, AUD-042, EN-147, `REF-DRISCOLL-1989`, `REF-MIT-6854-PERSISTENT`, `REF-CLOJURE-DATA-STRUCTURES`, `REF-IMMUTABLE-JS`

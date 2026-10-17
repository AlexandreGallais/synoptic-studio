# Research protocol

Goal: every function, rule or decision is justified by an identified source. This page says **where to look**, **in which order**, and **when to stop and ask**.

## 1. Search order (escalation)

| Level | Where | If insufficient |
| --- | --- | --- |
| 1 | `docs/domain/` | level 2 |
| 2 | `docs/adr/` | level 3 |
| 3 | `docs/derivations/`, `docs/references.md`, `docs/research/notes/` | level 4 |
| 4 | The local source library (`library_search`, `library_get`, [sources](../tooling/sources.md)), then primary external sources found with WebSearch | level 5 |
| 5 | Reference implementations, **read only** (see §4) | level 6 |
| 6 | **STOP.** Write a request in `research/requests/` and hand it to the Product Owner | — |

Every level-4 or level-5 finding is recorded in `research/notes/` and in `docs/references.md` **before** writing code.

## 2. Source reliability hierarchy

1. Normative specification (W3C, ISO, ISA)
2. Peer-reviewed paper
3. Reference book
4. Official software documentation (Figma, Inkscape, draw.io)
5. Open-source code
6. Blog, forum, course material — **never as the only source**

Two concurring sources of level ≥ 4 are required for an interface behavior rule.

### How many sources (RET-002)

No field of research sets one number: triangulation asks for sources that do not depend on each other, and quality counts more than count. Wikipedia's policy asks for "multiple high-quality sources" for an exceptional claim, without a number (`REF-WIKI-VERIFIABILITY`). The project sets its own rule, in tiers, and stops looking once the tier is met:

| What is claimed, from which level | Sources needed |
| --- | --- |
| a definition or a formula, from a level 1 to 3 source, read and quoted | one, its use checked by hand-computed tests or a derivation |
| a behavior or a convention, from levels 4 and 5 | two independent sources that agree |
| anything from level 6 | never alone: it only points to a source of a higher level |
| a surprising claim, or sources that disagree | three independent sources, or one of level 1 or 2; otherwise a research request (§5) |

Two sources are **independent** when neither cites nor copies the other and they do not come from the same origin (a vendor's page and a blog that repeats it are one source). An expert agent that finds the tier already met in the library still looks for one more independent site when the claim is new to the project; it stops there.

## 3. Topic map

| Topic | Internal first | External next |
| --- | --- | --- |
| Vectors, orientation, predicates | `math/` | `REF-SHEWCHUK-1997`, `REF-KETTNER-2008`, `REF-OROURKE` |
| Segment / arc intersections | — | `REF-SCHNEIDER-EBERLY`, `REF-BOURKE-CIRCLES` |
| Corner radius, fillet | ADR-0007, `DERIV-local-radius-clamp`, `DERIV-fillet-setback` | `REF-CSS-BR`, `REF-FIGMA-CR` |
| Regular polygon | `DERIV-regular-polygon-fit` | `REF-INKSCAPE-POLYGON` |
| Boolean operations | ADR-0006 | `REF-MARTINEZ-2009`, `REF-GREINER-HORMANN`, `REF-DEBERG`, `REF-CLIPPER2`, `REF-PAPERJS` |
| Offset, aligned strokes | ADR-0002 | `REF-SVG-STROKES`, `REF-SVG2-PAINT`, `REF-CHEN-MCMAINS-2005`, `REF-CLIPPER2` |
| SVG arcs (`A`) | — | `REF-SVG2-PATHS`, `REF-SVG2-IMPLNOTE` |
| Pipe routing | ADR-0004 | `REF-WYBROW-2009`, `REF-MARRIOTT-2014`, `REF-LIBAVOID` |
| P&ID / synoptic conventions | `symbols-and-views.md` | `REF-ISO-10628`, `REF-ISA-5-1`, `REF-ISA-101`, `REF-TOGHRAEI` |
| Snapping, alignment | `interaction.md` | `REF-INKSCAPE-SNAP` |
| Shape builder | ADR-0006 | `REF-INKSCAPE-SHAPEBUILDER` |
| Undo / redo | `interaction.md` | `REF-GOF` (Command pattern) |
| Tooling, code style | ADR-0009 to ADR-0016 | [note 0001](./notes/0001-tooling-practices.md) |

## 4. Licenses of reference implementations

| Project | License | Rule |
| --- | --- | --- |
| Inkscape, libavoid | GPL / LGPL | read to understand; **no copy, no line-by-line translation** |
| Clipper2, Paper.js | Boost / MIT | same rule on principle: reimplement from the paper or the spec |

Project code is always justified by a level 1 to 4 source or a derivation, never by "it is done this way in X".

## 5. Research request (level 6)

When the documentation is insufficient, the agent:

1. stops implementing;
2. copies `requests/_template.md` to `requests/NNNN-topic.md` and fills it in;
3. asks the Product Owner to run the research in Claude (chat, web research), see §6;
4. records the answer in `notes/NNNN-topic.md`, updates `docs/references.md`, then resumes.

## 6. Running a research in Claude (chat)

The agent does not always have web access, and a serious bibliographic search takes reading time. When a request is open (§5), the Product Owner runs it in **claude.ai**:

| Setting | Value |
| --- | --- |
| Model | the most capable one available (Claude Opus family) |
| Mode | **Research** enabled, to read online sources |
| Input | the "Precise question" section of the request, pasted as is, followed by the "Expected deliverable" |
| Requirement to restate | primary sources (spec, paper, book) with DOI or URL; say explicitly when a source was not read directly |

On receiving the answer, the agent:

1. summarizes it in its own words in `notes/NNNN-topic.md` (no copy of source text);
2. adds every source to `docs/references.md`: **`[verified] YYYY-MM-DD`** only if it was read (by the agent, through the local library or WebFetch, or attested as read by the answer); otherwise **`[unverified]`**;
3. sets the request status to "done".

## 7. Research done by the agent itself

With the local library (`library_search`, `library_get`) and WebSearch / WebFetch, the agent may do levels 4 and 5 without a request. Same recording: note in `notes/`, dated entry in `docs/references.md`, **before** the code. Example: [note 0001](./notes/0001-tooling-practices.md).

## 8. Feature research spike

The `SP` story opening every feature (ADR-0020), done **before** its implementation stories (procedure: the `/spike` skill):

1. List every formula, algorithm, specification point and interface behavior the feature's stories need.
2. For each one: an existing verified `REF-*` or `DERIV-*`, or a new source found and read (levels 4–5, recorded in `docs/references.md` with its date), or a derivation written, or a research request opened (§5) — never "to be found later".
3. Write a research note `notes/NNNN-<feature>.md`: what was found, for which story, what remains open.
4. Update the stories: acceptance criteria from the sources' examples, `@see` targets in the tasks, blocked points.
5. Ask the Product Owner the business questions found on the way (`docs/domain/README.md` open questions).

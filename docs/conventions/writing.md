# Writing documentation

Pages people scan, not read: the point first, one purpose per page, and a table, a list, a diagram or a code block wherever plain text would make the reader work. When in doubt, follow this page, then Google's guide (`REF-GOOGLE-DEV-STYLE`).

> [!NOTE]
> Readers scan: 79 % of test users "always scanned any new page"; text that is concise, scannable and objective was **124 % more usable** in the same study (`REF-NNG-SCANNING`). Every rule below serves scanning.

## Five rules

| Rule | In practice | Source |
| --- | --- | --- |
| **Point first** | the first sentence of a page or section says what it gives; conclusion before details (inverted pyramid) | `REF-NNG-SCANNING`, `REF-MS-STYLE` |
| **One page, one purpose** | a page is a concept, a task, a reference or a decision — never two (types below) | `REF-DIATAXIS`, `REF-GITLAB-TOPICS` |
| **Show, then tell** | comparisons in a table, steps in a numbered list, options in bullets, commands and data in code blocks, structures in a diagram | `REF-GOOGLE-LISTS`, `REF-GOOGLE-IMAGES` |
| **Short blocks** | paragraphs of one idea and at most four lines; a page longer than two screens gets headings every screen | `REF-NNG-SCANNING` |
| **One fact, one place** | link instead of repeating; `AGENTS.md` and folder indexes summarize and link | `REF-WTD-STYLE` |

## Page types

| Type | Answers | Here |
| --- | --- | --- |
| Concept (explanation) | "Why is it like this?" | `domain/`, ADRs, derivations, `guide/` pages |
| Task (how-to) | "How do I do X?" | `tooling/`, skills, "Changing a rule" sections |
| Reference | "What is it, exactly?" | API pages (TSDoc), `references.md`, tables in `conventions/` |
| Decision | "What did we choose, and why?" | ADRs, settled questions |

A reference page does not explain: it **links** to the explanation (`REF-DIATAXIS`). Titles say what the page holds, in fewer than 70 characters, never "Overview" or "Introduction" (`REF-GITLAB-TOPICS`).

## Building blocks

| Use | When | How |
| --- | --- | --- |
| **Table** | items sharing properties: options, comparisons, catalogs, rules with their source | one row per item, a first column naming it; no table of one column |
| **Numbered list** | steps whose order matters | one action per step, starting with a verb |
| **Bulleted list** | items without order | introduced by a complete sentence ending with a colon; never one item; parallel wording (`REF-GOOGLE-LISTS`) |
| **Bold run-in** | a few named ideas in a list | `- **Name**: what it is` |
| **Code block** | commands, data, file contents, text diagrams | always with a language (`sh`, `ts`, `json`, `text`) |
| **Diagram** | a flow, a structure, a lifecycle, a branch history; a geometry | a `mermaid` code block (flowchart, gitGraph, sequenceDiagram): text kept with the page, drawn by the site and by GitHub (`REF-MERMAID`, `REF-GH-DIAGRAMS`), parsed by `tests/diagrams.test.ts` so that a broken diagram fails the check, label lines of at most 32 characters (cut with `<br/>`) so that the text fits its box; a geometry: SVG drawn by the library in an `images/` folder; introduced by a sentence |
| **Alert** | what a scanning reader must not miss | `> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, `> [!WARNING]`: rendered by GitHub and VitePress (`REF-VITEPRESS-MARKDOWN`); one or two per page at most |
| **Paragraph** | reasoning that needs sentences: context, why, trade-offs | short; the point in its first words |

Images (`REF-GOOGLE-IMAGES`): SVG first; alt text short and descriptive, under 155 characters, never "Image of"; no text in images that the page does not also say; an image only where words are not enough.

## Writing what the Product Owner said

The Product Owner speaks freely while dictating; the documentation keeps their **decisions and their reasons**, never their words.

| Do | Do not |
| --- | --- |
| write the decision as understood, in complete neutral English | quote them, in French or in English |
| attribute it to the role and the date: "Product Owner decision, 2026-10-11" | write feelings, praise or familiar words |
| give the reason when it was given: "so that a width of 0 then back gives the same shape" | cut it short to one line when the rule needs a paragraph |
| structure a long decision: a heading, a list of the rules, a table of the cases | write it as a run of one-line bullets with no order |
| ask when the meaning is unclear | guess and record the guess as their decision |

The same holds for commit messages, the improvement and feedback logs, retrospectives, and messages to them (written in French, same shape).

## Writing a rule

A rule is written as a principle, never as the transcript of the conversation that decided it: the reader must be able to apply it to a case nobody discussed.

| Part | What it says | Example |
| --- | --- | --- |
| principle | the idea in general terms, in one sentence | "A function never modifies its arguments." |
| reason or test | why it holds, or how to tell whether a case falls under it | "A caller must be able to reuse a value it passed." |
| on conflict | which rule wins, or whom to ask, when it meets another | "A procedure may still call a DOM element's methods: changing the DOM is its job." |

- Give examples only where they remove a doubt, and then a range of them: the usual case, a case at the edge, and a case outside the rule.
- Record when and by whom a rule was decided as a date and a role (`Product Owner, 2026-10-17`), never in place of its reason.
- Keep a rule only if it makes the code or the docs more correct or more uniform. A rule that only adds writing, or repeats what a tool already enforces, is cut.
- One rule lives in one place; the other pages link it.

## Style

- English, present tense, active voice, second person for instructions ("run `npm run fix`").
- Sentence-case headings, no end punctuation; one `#` title per page; serial comma (`REF-MS-STYLE`, `REF-GOOGLE-DEV-STYLE`).
- Start an instruction with its verb; cut "you can", "there is", "simply", "just", "obviously".
- Code, file names, commands and identifiers in `code font`; dates as `YYYY-MM-DD`.
- **Same thing, same word**: the word list below; never vary a term for style.
- Reference and convention pages: austere, no contractions. Guide pages: plain language, contractions allowed, every term defined at first use (`REF-PLAIN-LANGUAGE`).
- House style: the em dash keeps a space on each side ( — ).

## API reference (TSDoc)

Clicking a function shows only what is needed to use it:

1. one sentence saying what it returns or does;
2. the formula, when there is one;
3. `@param` / `@returns` fragments (unit, range, convention: `radians, in ]0, π[`);
4. `@see` to the source — the reader follows it for the "why" and the "how".

No tutorial, no history, no design discussion, no restating of the name (`jsdoc/informative-docs`): those belong in an ADR, a derivation or the domain, cited by `@see`.

## Word list

The [glossary](../domain/) defines the domain terms; this list fixes the words used to write about them.

| Use | Not | Note |
| --- | --- | --- |
| corner radius | border radius, rounding | the requested value; "effective radius" for the reduced one |
| fillet | round, curve | the arc replacing a corner |
| setback | offset, inset | distance from a corner to a tangent point |
| contour | outline, polygon (for the list of vertices) | closed list of vertices, clockwise |
| vertex, vertices | point, node (in geometry) | "point" in the playground and the guide, where the user sees points |
| path data | `d` string, path string | the `d` attribute of `<path>` |
| side | edge (for a free shape with handles) | the part of the contour between two points; "curved side" when cubic |
| Mirror angle and length, Mirror angle, No mirroring | mirrored angle and length, mode angle | the handle modes as the playground names them; stored `angle-and-length`, `angle`, `none` |
| method (of typing a handle) | way | `HANDLE_WAYS` in the code |
| Product Owner | PO, client, user (for the Product Owner) | "user" means the end user of the tools |
| story, task | ticket, issue (for backlog items) | "issue" only for GitHub issues |
| pull request | PR, merge request |  |
| playground | demo app, sandbox |  |

## Page skeletons

| Page | Sections, in order |
| --- | --- |
| folder `README.md` | one sentence of purpose; table of pages |
| ADR | Status; Context; Decision; Reasons (options table); Consequences; References |
| derivation | statement and domain; steps, each citing a read source; cross-check; check table |
| research note | date and origin; findings per topic with `REF-*`; retained table |
| guide page | template `PAGE-guide-epic-report` |
| backlog item | templates `docs/backlog/_*.md` |

## Messages to the Product Owner

In French, the same shape every time:

1. **Result** in one sentence: done, partly done, or blocked.
2. **What changed**: a table or bullets, the visible effect first; each technical word explained at first use.
3. **Evidence**: checks run and their result, links to the pull request or commits.
4. **Decisions needed**: numbered questions, each answerable by yes / no or a choice, with a recommendation.
5. **Next step**, and what waits for the Product Owner.
6. **Session report**, at a session's end only: `target/release/session-report --all --markdown` (time and tokens of every turn of the session).

No raw logs: quote the one line that matters. Questions come last, so they are not lost.

## Checked by the linter

Markdown pages are linted by the project's linter (`tools/lint`, [lint.md](./lint.md#markdown-rules)): heading levels, one `#` title, code block languages, no bare URL, no missing link fragment, table column counts, alternative texts. A heading a link targets has one anchor on GitHub and on the site (`local/anchor-slugs`): the site turns `'`, `/` and `.` into hyphens where GitHub drops them, `_` into a hyphen where GitHub keeps it, keeps `—` where GitHub drops it, and puts `_` before a leading digit, so such a heading is reworded (`The settings of the formatter`, not `The formatter's settings`), a rule's name moving to the section's first line. Layout is the formatter's (oxfmt, tables without padding: `formatting.md`).

## Where things go

| Content | Place |
| --- | --- |
| business rule | `docs/domain/` |
| structural decision and its alternatives | new ADR |
| an ADR that no longer holds | its row in the record of ADR-0101, its file removed |
| code rule | `docs/conventions/` (and the lint rule enforcing it) |
| tooling procedure | `docs/tooling/` |
| planned work | `docs/backlog/` |
| source | `PAGE-references` row, cited by ID |

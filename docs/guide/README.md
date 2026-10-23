# Guide

What the library does, explained in plain language for the Product Owner and, later, for users. No formula without a sentence that says what it means; no term without a definition or a link to the [glossary](../domain/). Each page ends with **how to try it** in the playground and the **validation** by the Product Owner.

Written at the end of each feature (`VAL`, demo page) and of each epic (`REV`, epic report), ADR-0023. Category: explanation (`REF-DIATAXIS`); the API reference stays austere.

## Writing rules

1. Start with what the user sees, then why, then how — one idea per paragraph.
2. Explain each technical word the first time: "a **fillet** is the small arc that replaces a sharp corner".
3. Numbers from the playground rather than symbols: "a 100 × 25 rectangle with radius 100 gets radius 12.5".
4. A picture (SVG produced by the library, or a sketch) for every geometric idea.
5. Say the limits: what the feature does not do yet, and which feature will.
6. A feature is validated on its epic's **checklist**, one page in the playground (`?checklist=E15`): one line per option kept and per decision to confirm, what to try and what to see, a check box before it, no Previous nor Next; the boxes ticked stay in that browser (US-051). The Product Owner says what they see; the agent records the result and the remarks in neutral words (`docs/conventions/writing.md`).
7. The guided steps written up to E15 stay in the playground's epic list, `?guide=epic`, for a closer look; they are no longer the validation path.

## Pages

| Page | Content |
| --- | --- |
| [F01 — A rectangle with rounded corners](./f01-rounded-rectangle.md) | rounding corners, the radius reduced to fit, the guided test |
| [F02 — A regular polygon with rounded corners](./f02-regular-polygon.md) | the polygon fitted in its box, the inscribed circle, sizes, the guided test |
| [F07 — A polygon anchored in its box](./f07-polygon-anchor.md) | the nine anchor buttons, the intent kept, rounding inwards, the guided test |
| [F03 — A rounding for each corner](./f03-radius-per-vertex.md) | points mode, point 1 and 2, choosing corners, Mixed, the guided test |
| [F09 — The rectangle, a free shape you can stretch](./f09-stretchable-free-shape.md) | points in a box of their own, stretching without loss, the guided test |
| [F08 — Moving, adding and deleting points](./f08-node-editing.md) | converting a polygon, typing x and y, a point in the middle, deleting, the guided test |
| [F04 — Turning a shape by any angle](./f04-free-rotation.md) | the angle, the two boxes, the position, editing a turned shape, the guided test |
| [F06 — A polygon stretched to its box](./f06-stretched-polygon.md) | the **Stretched to its box** check box, rule C in five boxes, the guided test |
| [F11 — A code base ready for curves](./f11-code-base-ready-for-curves.md) | the figures before and after, the folders by domain, how to check them |
| [F12 — Points on a curve, with handles](./f12-curves-with-handles.md) | the options of handles, a corner beside a curve, the guided test, what to decide |
| [F10 — Round shape](./f10-round-shape.md) | the Round tool, its two conversions, the guided test, what the agent decided |
| [F13 — Every calculation named](./f13-every-calculation-named.md) | the calculations of the code named after their mathematics, every drawing unchanged |
| [F05 — Text and its characters](./f05-text.md) | the Text tool and its fields, the measured block, turning, the guided test, what to confirm |
| [F14 — A box of text](./f14-text-box.md) | wrapping in a box, the ellipsis, both alignments, line height, trim, the background, the guided test, what to confirm |
| [F16 — Colors and their picker](./f16-colors.md) | the hexadecimal forms, the swatch and picker, HSB and HSL, the guided test, what to confirm |
| [F15 — Fonts from files and embedded fonts](./f15-fonts.md) | fonts imported or installed, embedding and its reasons, the export, the guided test, what to confirm |
| [F17 — Text corrections after the E15 validation](./f17-text-corrections.md) | the options removed, the defects fixed and why, the checklist, the readings and questions to confirm, Apple Color Emoji |
| [F18 — A safe and cheap parallel run](./f18-safe-and-cheap-parallel-run.md) | the limits the tools enforce, the figures before and after, each criterion's evidence, the commands to try, the decisions confirmed |
| [F19 — Checks on Oxc, TypeScript 7 and Rust](./f19-checks-on-oxc-and-rust.md) | the checks moved to native tools, the time before and after, the rules triaged, the tests made leaner, the ADRs, the audit, the commands to try |
| [F20 — A context diet for every agent](./f20-a-context-diet.md) | an agent type per task, the first requests and the rules' size before and after, the models and their trial, the Git sequences and the library, the hand-back check, a run's totals and stops, the audit, the commands to try |
| [F21 — Agent tools and the library server in Rust](./f21-agent-tools-in-rust.md) | every hook, script, Git hook and check of our own in Rust, the time per tool before and after, the packages removed, the release by our own tool and its notes against release-please's, the record of each tool's use, the audit, the decisions to confirm, the commands to try |
| [F22 — One story at a time, the release by a button, its CI green](./f22-release-by-a-button.md) | the CI green on GitHub's runner, the release by a button from the tags and its notes the features' texts, one story at a time and a merge commit for every merge, a review at each return, the audit, the decisions to confirm, the commands to try, what waits for the next review |
| [F24 — The CI in parallel jobs, its tests shown as they run](./f24-the-ci-in-parallel-jobs.md) | five lane checks in parallel, the tests file by file with their failures annotated, the Rust cache saved from `develop` in the background, the warm time pending |
| [F28 — One workflow per purpose](./f28-one-workflow-per-purpose.md) | the Actions tab's workflows one per purpose, the site apart from the release, the CI into `main` with only the steps its lanes need |
| [F29 — The local checks stable and fast](./f29-local-checks-stable-and-fast.md) | timing limits at four times their measure, the Rust lanes skipped without a Rust change, the binaries rebuilt after a pull |
| [F30 — A CI on develop that blocks nothing](./f30-a-ci-on-develop-that-blocks-nothing.md) | the Rust tests on Linux and the caches after each merge into `develop`, dispatched by the auto-merge, blocking nothing |
| [F23 — The path of a story through the tools](./f23-the-path-of-a-story.md) | the log rows and cards in each story's `done` commit, nothing between the bubbles; a review or an epic's refinement through the same tools |
| [F31 — The guard and the agent's commands](./f31-the-guard-and-the-agents-commands.md) | the guard reading by words, two ways past it closed, writes outside a worktree refused, `git-sequences state`, `replace`'s usage, a types-only barrel line kept |
| [F32 — Reports that say what happened](./f32-reports-that-say-what-happened.md) | ended agents no longer shown running, the command after `\|\| exit 9` named, a session's tokens by model, `epic-check` on the CI into `main`, durations rounded, struck criteria left out |
| [F33 — The checks RET-004 listed](./f33-the-checks-ret-004-listed.md) | the coverage floor at 95 %, `typeCheck` in oxlint's configuration, the cargo tools' versions, one anchor per heading, the notes `move-rows` leaves, a `bench` binary judged by the guard |
| [F34 — The use of each workflow](./f34-the-use-of-each-workflow.md) | the auto-merge split by base, the outside pull requests' workflow removed, CodeQL on the workflows, the link check kept |
| [F35 — Several shapes on one drawing](./f35-several-shapes-on-one-drawing.md) | a click with a tool adds a shape where it is clicked, a new shape with the settings of the last shape of its kind |
| [F36 — Draw and pick shapes](./f36-draw-pick-and-remove-shapes.md) | a drag with a tool draws the new shape at the size dragged, square with Shift; the shape under the pointer outlined; a click with the Selector picks a shape for the panel |
| [F37 — Select shapes](./f37-select-shapes.md) | the default keys of selecting, deleting, nudging and undoing, Figma's where its help names them |
| [F38 — The panel for several shapes](./f38-the-panel-for-several-shapes.md) | the position and size of several shapes, each shape's values, Mixed where they differ, a turned shape by its straight box, a text by its frame |
| [E01 — Drawing symbol shapes from numbers](./e01-draw-shapes.md) | the eight features of E01, the words, a user test crossing them, the decisions |
| [E14 — Drawing curves with handles, ellipses and round shapes](./e14-draw-curves.md) | the four features of E14, the words, the walk-through of the review, the decisions |
| [E15 — Placing text in symbols](./e15-place-text.md) | the five features of E15, the words, the tour of the review, the decisions, the direction to remove |
| [E17 — Fast tooling with Oxc and Rust](./e17-fast-tooling-with-oxc-and-rust.md) | the four features of E17 with NAM-002, EN-114 and EN-115, the figures before and after, the words, a story's life as the user test, the decisions, the checklist run again, the questions of the review |

Templates: `_feature-demo.md`, `_epic-report.md` (not published).

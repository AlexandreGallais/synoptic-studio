---
id: E50
title: Share the agent's setup with every coding agent
status: draft
---

# E50 — Share the agent's setup with every coding agent

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the skills, rules and agent briefs readable by any coding agent, not only Claude Code. Asked by the Product Owner on 2026-10-25, after REV-009; after E49, before E21.

**The Product Owner's account** (2026-10-25, in neutral words): `.claude/` holds skills, rules and agents that only Claude Code reads. Their content moves to a folder every agent reads, `.agents/` for instance, and `.claude/` keeps what Claude Code alone needs, its settings first; `AGENTS.md` keeps the rules for every agent and `CLAUDE.md` only what is Claude Code's. The agent may rework what `.claude/` holds where it needs it; its means of work are kept.

| Field | Content |
| --- | --- |
| For | the Product Owner, who may change tools |
| who | wants every strong coding agent to work here as well as Claude Code |
| the solution | a folder of the agent's method shared by every agent |
| is a | `.agents/`: skills, rules and briefs as plain files |
| that | any agent reads, `AGENTS.md` pointing to it |
| unlike | `.claude/`, read by Claude Code alone |
| our solution | the content in `.agents/`, `.claude/` keeping links to it and the settings |
| Business outcomes | another agent finds the method without Claude Code |
| Leading indicators | the files under `.claude/` that are not settings nor links |
| Non-functional requirements | Claude Code loses no skill, rule, agent nor hook |
| In scope | `.agents/`; the links from `.claude/`; `AGENTS.md` and `CLAUDE.md` cut to their roles |
| Out of scope | the knowledge database (E49) |

## Features

Proposed when the epic was written (2026-10-25) as drafts: its refinement cuts them again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The shared folder | what Claude Code follows through a link, read and tried; the skills, rules and briefs moved, `.claude/` linking to them (2) | to refine |
| The two entry pages | `AGENTS.md` for every agent, `CLAUDE.md` for Claude Code's own (2) | to refine |

Size: 2 features, 4 stories, 20 + 25 × 4 + (45 to 75) × 2 = 210 to 270 active minutes, 3.5 to 4.5 hours (note 0034, §1).

## Open points

Whether Claude Code reads skills, rules and agents through a symbolic link, to read and try at its refinement.

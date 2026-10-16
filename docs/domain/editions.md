# Editions — free V1 and pro edition

Where the project stops (ADR-0030): a finished free V1, then new features only in a paid pro edition.

## 1. Free edition (V1)

- Everything delivered before the V1 release, in this public repository, under the Apache License 2.0.
- **Every epic of the backlog** (`docs/backlog/epics/`) makes V1 (Q22, Product Owner, 2026-10-11): E11 saves and reloads files in the working folder, without any server; E07 publishes business libraries locally, never online. An epic for a design system may come later, once the Product Owner can describe it. Features inside each epic are still set along the way.
- **Local only** (Product Owner, 2026-10-11): it runs on the user's computer; like a desktop vector editor, the user opens a **working folder** and saves files one by one. No server side at all — no local server, no online service (Q5): the V1 is a local application, nothing more.
- Stays as simple as possible: synoptic views, not drawing.
- **Application shell** (open, Q23): a web view in a desktop app — all computation in the web view, native code only for saving files; Electron or a Rust-based desktop framework, to compare when the time comes.
- After V1: bug fixes only.

## 2. Pro edition — ideas kept, out of V1

A private, paid product of the Product Owner, sold as a service or self-hosted, with every free feature and more. Nothing here is refined nor planned in this backlog; each idea is dated. Pro ideas are written here, in the public repository, on purpose: sharing advanced ideas openly is the Product Owner's choice (2026-10-11).

| Idea                                  | What it does                                                                                                                                                                                                                                                                                                                                                       | Given      |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------- |
| Symbol change report                  | in the symbol test system, compares the symbol before and after a change and reports the SVG difference, explained in plain words, such as "the background color of this element changed"                                                                                                                                                                          | 2026-10-11 |
| Open API on every element             | next to the configured properties that drive a symbol in the front end, an open API: a third-party service (a back end sending data orders to the front end) says "this element turns by this much" without any property; the user chooses which way to work and can keep all their rules in their own service                                                     | 2026-10-11 |
| Project management, servers           | projects managed across files, on a server or on local servers, instead of one working folder and files saved one by one; the local server first planned for V1 (E11, Q5 before 2026-10-11) belongs here                                                                                                                                                           | 2026-10-11 |
| Faster engine without business limits | the core rewritten in Rust and compiled to WebAssembly; fast enough to drop the limits the free edition sets for speed and simplicity and handle every reasonable case of the mathematics and geometry — for example very large shapes, beyond the million units where the free edition warns and offers to simplify (Q20, E11); the model is unchanged (ADR-0025) | 2026-10-11 |
| Full drawing interface                | a second interface for real, advanced drawing, like a desktop vector editor, next to the synoptic symbol and view editors, built on the same engine                                                                                                                                                                                                                | 2026-10-11 |
| Online service (SaaS)                 | maybe later, not planned for now                                                                                                                                                                                                                                                                                                                                   | 2026-10-11 |
| Text styles enforced                  | named text styles (title, body, description) set by an administrator, that every designer must follow; the free edition keeps text styles as local templates that forbid nothing (Product Owner, 2026-10-14, F05 refinement)                                                                                                                                       | 2026-10-14 |

## 3. Telling them apart

A feature goes to the pro edition when companies want it and an individual user does not, or when it goes beyond editing synoptic symbols and views (testing, integration with other services, project management, servers, performance). When unsure, the agent asks the Product Owner (ADR-0030).

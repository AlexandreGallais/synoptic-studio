# Editions — free V1 and pro edition

Where the project stops (ADR-0030): a finished free V1, then new features only in a paid pro edition.

## 1. Free edition (V1)

- Everything delivered before the V1 release, in this public repository, under the Apache License 2.0.
- **Every epic of the backlog** (`docs/backlog/epics/`) makes V1 (Q22, Product Owner, 2026-10-11): E11 saves and reloads files in the working folder, without any server; E07 publishes business libraries locally, never online. An epic for a design system may come later, once the Product Owner can describe it. Features inside each epic are still set along the way.
- **Local only** (Product Owner, 2026-10-11): it runs on the user's computer; like a desktop vector editor, the user opens a **working folder** and saves files one by one. No server side at all — no local server, no online service (Q5): « pour la V1, on ne va sortir qu'un logiciel en local et point barre. »
- Stays as simple as possible: synoptic views, not drawing.
- **Application shell** (open, Q23): « Il y aura un serveur web à lancer, potentiellement. Ce sera juste une web view » — all computation in the web view, native code only for saving files; Electron or a Rust-based desktop framework, to compare when the time comes.
- After V1: bug fixes only.

## 2. Pro edition — ideas kept, out of V1

A private, paid product of the Product Owner, sold as a service or self-hosted, with every free feature and more. Nothing here is refined nor planned in this backlog; each idea is quoted when it was given. Pro ideas are written here, in the public repository, on purpose: « je m'en fous si je partage des trucs avancés » (Product Owner, 2026-10-11).

| Idea                        | What it does                                                                                                                                                                                                                                                                                                               | Given      |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| Symbol change report        | in the symbol test system, compares the symbol before and after a change and reports the SVG difference, explained in plain words: « La couleur de fond de tel élément a changé. »                                                                                                                                         | 2026-10-11 |
| Open API on every element   | next to the configured properties that drive a symbol in the front end, an open API: a third-party service (a back end sending data orders to the front end) says « tel élément fait une rotation de tel truc » without any property; the user chooses which way to work and can keep all their rules in their own service | 2026-10-11 |
| Project management, servers | projects managed across files, on a server or on local servers, instead of one working folder and files saved one by one; the local server first planned for V1 (E11, Q5 before 2026-10-11) belongs here                                                                                                                   | 2026-10-11 |
| Faster engine               | the core transcribed to Rust and compiled to WebAssembly if more speed is needed, without changing the model (ADR-0025 keeps this possible)                                                                                                                                                                                | 2026-10-11 |
| Online service (SaaS)       | maybe later: « on n'y est pas pour l'instant »                                                                                                                                                                                                                                                                             | 2026-10-11 |

## 3. Telling them apart

A feature goes to the pro edition when companies want it and an individual user does not, or when it goes beyond editing synoptic symbols and views (testing, integration with other services, project management, servers, performance). When unsure, the agent asks the Product Owner (ADR-0030).

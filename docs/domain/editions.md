# Editions — free V1 and pro edition

Where the project stops (ADR-0030): a finished free V1, then new features only in a paid pro edition.

## 1. Free edition (V1)

- Everything delivered before the V1 release, in this public repository, under the Apache License 2.0.
- **Every epic of the backlog** (`docs/backlog/epics/`) makes V1 (Q22, Product Owner, 2026-10-11): files are saved and reloaded without any server, from the web page (E41) and in the working folder of the desktop application (E46); business libraries are published locally, never online (E37). An epic for a design system may come later, once the Product Owner can describe it. Features inside each epic are still set along the way.
- **Local only** (Product Owner, 2026-10-11): it runs on the user's computer; like a desktop vector editor, the user saves files one by one, in a **working folder** once the desktop application comes (E46, Q23). No server side at all — no local server, no online service (Q5): the V1 is a local application, nothing more.
- **A web page first** (Product Owner, 2026-10-25, Q23): the editor works first as a web page, in every browser. A file is opened from the disk and saved by **Save as**, a download, which stands in for the working folder; Ctrl + S saves in place where the browser allows it (the File System Access API, to source at E41's refinement), otherwise it opens Save as. A file is saved only when the designer asks, never continuously (E41).
- Stays as simple as possible: synoptic views, not drawing.
- **Application shell** (Q23, settled on 2026-10-25): after the web page, a desktop application, an epic of its own (E46) placed just before the release's epics, likely a Rust program hosting a web view, how being researched at that epic's refinement; all computation stays in the web view, native code only for the files; the working folder comes with it.
- **On GitHub only** (Q40, Product Owner, 2026-10-17): the library is never published on npm.
- **English only, no accessibility layer** (Q41, Product Owner, 2026-10-17): no other language and no `title` nor `desc` in V1; synoptics are read on factory screens, not by screen readers.
- After V1: bug fixes only.

## 2. Pro edition — ideas kept, out of V1

A private, paid product of the Product Owner, sold as a service or self-hosted, with every free feature and more. Nothing here is refined nor planned in this backlog; each idea is dated. Pro ideas are written here, in the public repository, on purpose: sharing advanced ideas openly is the Product Owner's choice (2026-10-11).

| Idea | What it does | Given |
| --- | --- | --- |
| Symbol change report | in the symbol test system, compares the symbol before and after a change and reports the SVG difference, explained in plain words, such as "the background color of this element changed" | 2026-10-11 |
| Open API on every element | next to the configured properties that drive a symbol in the front end, an open API: a third-party service (a back end sending data orders to the front end) says "this element turns by this much" without any property; the user chooses which way to work and can keep all their rules in their own service | 2026-10-11 |
| Project management, servers | projects managed across files, on a server or on local servers, instead of one working folder and files saved one by one; the local server first planned for V1 (E11, Q5 before 2026-10-11) belongs here | 2026-10-11 |
| Faster engine without business limits | the core rewritten in Rust and compiled to WebAssembly; fast enough to drop the limits the free edition sets for speed and simplicity and handle every reasonable case of the mathematics and geometry — for example very large shapes, beyond the million units where the free edition warns and offers to simplify (Q20, E42); the model is unchanged (ADR-0025) | 2026-10-11 |
| Full drawing interface | a second interface for real, advanced drawing, like a desktop vector editor, next to the synoptic symbol and view editors, built on the same engine | 2026-10-11 |
| Online service (SaaS) | maybe later, not planned for now | 2026-10-11 |
| Text styles enforced | named text styles (title, body, description) set by an administrator, that every designer must follow; the free edition keeps text styles as local templates that forbid nothing (Product Owner, 2026-10-14, F05 refinement) | 2026-10-14 |
| Accessibility and other languages | `title` and `desc` for screen readers, and the editors and symbols in other languages than English; out of V1 (Q41) | 2026-10-17 |
| Contrast of texts | a text's color measured against what is behind it, in symbols, views and their animations: two results side by side, the WCAG 2 contrast ratio with its levels AA and AAA and the large-text thresholds by size and weight, and APCA, the WCAG 3 draft's lightness contrast, read by size and weight; a verdict in plain words for the text's size and weight, such as readable or too pale for a bold 16 px text; a minimum contrast required at all times, whatever colors the text and its background take through animations; sources to read when it is refined, none read yet: WCAG 2.2's success criteria 1.4.3 and 1.4.6, and APCA (given on 2026-10-15 as E16; placed entirely in the pro edition by the Product Owner on 2026-10-25, SP-033) | 2026-10-15 |

## 3. Telling them apart

A feature goes to the pro edition when companies want it and an individual user does not, or when it goes beyond editing synoptic symbols and views (testing, integration with other services, project management, servers, performance). When unsure, the agent asks the Product Owner (ADR-0030).

# Editions — free V1 and pro edition

Where the project stops: a finished free V1, then new features in a paid pro edition only.

## 1. Free edition (V1)

- **What it holds**: everything delivered before the V1 release, in this public repository, under the Apache License 2.0; every epic of the backlog ([the order of the epics](../backlog/)), the features inside each still set along the way. Saving files without a server is in V1.
- **Local only**: it runs on the user's computer; files are saved one by one, in a working folder once the desktop application comes; no server side at all, no local server, no online service. A business library is published locally, never online.
- **A web page first**, in every browser: a file opened from the disk and saved by **Save as**, a download standing in for the working folder; Ctrl + S saves in place where the browser allows it, otherwise opens Save as; a file saved only when the designer asks, never continuously. Ctrl + S in a browser without in-place saving opens Save as.
- **Then a desktop application**, an epic of its own just before the release, likely a Rust program hosting a web view: all computation stays in the web view, native code only for the files; the working folder comes with it.
- **As simple as possible**: synoptic views, not drawing.
- **On GitHub only**: the library is never published on npm.
- **English only, no accessibility layer**: no other language, no `title` nor `desc`, synoptics being read on factory screens, not by screen readers.
- **After V1**: bug fixes only.
- **A design system epic** may be added later, once it can be described.

## 2. Pro edition — ideas kept, out of V1

A private, paid product, sold as a service or self-hosted, with every free feature and more. Nothing here is planned in this backlog; the ideas are written in the public repository on purpose.

| Idea | What it does |
| --- | --- |
| Symbol change report | in the symbol test system, compares a symbol before and after a change and explains the SVG difference in plain words, such as "the background color of this element changed" |
| Open API on every element | beside the configured properties, a third-party service tells the front end "this element turns by this much" without any property; the user chooses which way to work |
| Project management, servers | projects managed across files, on a server or local servers, instead of one working folder and files saved one by one |
| Faster engine without business limits | the core in Rust compiled to WebAssembly, fast enough to drop the free edition's limits, such as very large shapes beyond a million units; the model unchanged |
| Full drawing interface | a second interface for advanced drawing, like a desktop vector editor, on the same engine |
| Online service (SaaS) | maybe later, not planned |
| Text styles enforced | named text styles set by an administrator that every designer must follow; the free edition keeps them as local templates that forbid nothing |
| Accessibility and other languages | `title` and `desc` for screen readers, and the editors and symbols in other languages |
| Contrast of texts | a text's color measured against what is behind it, through animations too: the WCAG 2 contrast ratio with its levels AA and AAA and the large-text thresholds, and APCA's lightness contrast, side by side; a verdict in plain words for the text's size and weight; a minimum contrast required at all times |

## 3. Telling them apart

A feature goes to the pro edition when companies want it and an individual user does not, or when it goes beyond editing synoptic symbols and views: testing, integration with other services, project management, servers, performance. When unsure, the agent asks the Product Owner. A symbol test report is pro; a new shape tool is V1.

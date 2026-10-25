# Tooling

How the project is built, checked, documented, versioned and released, and how the agent works on it.

| Page | Content |
| --- | --- |
| [commands.md](./commands.md) | npm scripts, the full check's lanes, tests that run Git, the heavy lock, build |
| [dependencies.md](./dependencies.md) | every package, crate, tool and asset, why it is there, how to add one |
| [versions-and-security.md](./versions-and-security.md) | latest versions, upgrades, vulnerabilities |
| [git-workflow.md](./git-workflow.md) | branches, pull requests, commits, hooks |
| [ci.md](./ci.md) | the CI into `main`, the `develop` CI, the auto-merges, the Rust caches, rulesets, repository settings |
| [releases.md](./releases.md) | the features' texts, the version from the tags and the GitHub release by a button, by `tools/release` |
| [documentation-site.md](./documentation-site.md) | VitePress site, API pages, playground |
| [agent.md](./agent.md) | the agent's configuration: files, skills, agent types and models, hooks, parallel sessions |
| [agent-tools.md](./agent-tools.md) | the agent's own tools, the Rust workspace, the shared files |
| [metrics.md](./metrics.md) | how long the agent's work takes and how many tokens it uses; the status line |
| [sources.md](./sources.md) | the local source library (ADR-0039) |
| [knowledge.md](./knowledge.md) | the project's records in one SQLite file, its knowledge tables, two worktrees writing at once (E49) |

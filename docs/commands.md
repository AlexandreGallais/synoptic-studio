# Commands

What a person runs in a clone of the repository, with Node.js 24 or later and the Rust toolchain that `rust-toolchain.toml` names.

| Command | Does |
| --- | --- |
| `npm install` | installs the development packages, builds the project's Rust tools and sets the Git hooks |
| `npm run dev` | serves the playground on `http://localhost:5173` |
| `npm run docs:dev` | serves this site, its API pages written first |
| `npm run test` | runs the tests with their coverage |
| `npm run fix` | formats the files and applies the linters' fixes |
| `npm run check:all` | runs every check: format, lint, types, tests, the docs build, unused code, secrets, the dependencies' audit; the Git hook runs it before each push |
| `target/release/check-all --lane <name>` | runs one lane of the checks, as a job of the CI does: `lint`, `test`, `docs` (builds this site and the playground into it), `files`, `rust`, `rust-coverage`, `timings`; `--list` prints each lane's commands |
| `target/release/dependencies outdated` | lists the packages, crates, actions and tools behind their latest version at least 7 days old |
| `npx vitest` | runs the tests in watch mode |

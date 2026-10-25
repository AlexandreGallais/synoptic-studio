# Commands

What a person runs in a clone of the repository, with Node.js 24 or later and the Rust toolchain that `rust-toolchain.toml` names.

| Command | Does |
| --- | --- |
| `npm install` | installs the development packages, builds the project's Rust tools and sets the Git hooks |
| `npm run dev` | serves the playground on `http://localhost:5173` |
| `npm run docs:dev` | serves this site, its API pages written first |
| `npm run test` | runs the tests with their coverage |
| `npm run fix` | formats the files and applies the linters' fixes |
| `npm run lint` | runs the linters and the type check, changing nothing |
| `npm run check:all` | runs every check: format, lint, types, tests, the docs build, unused code, secrets, the dependencies' audit; the Git hook runs it before each push |
| `npm run build` | builds the library into `dist/`: one ES module and its type declarations |
| `npm run preview` | serves the playground as built |
| `npm run docs:build` | builds this site |
| `npm run playground:build` | builds the playground into the site |
| `npm run deps:outdated` | lists the packages, crates, actions and tools behind their latest version at least 7 days old |
| `npx vitest` | runs the tests in watch mode |

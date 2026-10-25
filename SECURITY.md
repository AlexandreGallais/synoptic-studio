# Security policy

## Reporting a vulnerability

Please report vulnerabilities **privately**, through GitHub: _Security → Report a vulnerability_ on this repository. Do not open a public issue.

## Scope

- **The published library** has no runtime dependency: `package.json` lists development packages only.
- **The development packages and crates** are audited before each push and in the CI (`npm audit`, cargo-deny against the RustSec database), and on GitHub by Dependabot, the dependency review, CodeQL and secret scanning.
- **Secrets**: the repository holds none; a secret scan runs before each push.

// npm-check-updates: every dependency must be on its latest version published at least 7 days ago
// (`npm run deps:outdated`, cooldown of ADR-0038: no red check for a version minutes old, and a
// guard against compromised fresh versions).
// `typescript` is TypeScript 6's API under the alias `@typescript/typescript6`, kept for
// typescript-eslint until it leaves (EN-071, EN-085): patch upgrades only.
module.exports = {
  target: (name) => (name === "typescript" ? "patch" : "latest"),
  cooldown: 7,
  errorLevel: 2,
};

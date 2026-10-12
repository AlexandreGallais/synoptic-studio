// npm-check-updates: every dependency must be on its latest version published at least 7 days ago
// (`npm run deps:outdated`, cooldown of ADR-0038: no red check for a version minutes old, and a
// guard against compromised fresh versions).
// TypeScript stays on 6.0.x until typescript-eslint supports 7 (ADR-0009): patch upgrades only.
module.exports = {
  target: (name) => (name === "typescript" ? "patch" : "latest"),
  cooldown: 7,
  errorLevel: 2,
};

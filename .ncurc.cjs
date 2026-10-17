// npm-check-updates: every dependency must be on its latest version published at least 7 days ago
// (`npm run deps:outdated`, cooldown of ADR-0038: no red check for a version minutes old, and a
// guard against compromised fresh versions).
module.exports = {
  target: "latest",
  cooldown: 7,
  errorLevel: 2,
};

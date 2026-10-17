import type { TSESLint } from "@typescript-eslint/utils";

/** Web platform APIs: fetch, URLs, HTML attributes, CSS, Markdown. */
export const UNICORN_WEB: TSESLint.FlatConfig.Config = {
  name: "unicorn/web",
  rules: {
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-blob-to-file": "off",
    "unicorn/no-conflicting-constraints": "error",
    "unicorn/no-document-cookie": "error",
    // Targets JSON or Markdown, which ESLint does not lint here (Prettier formats them).
    "unicorn/no-empty-link-text": "off",
    "unicorn/no-ineffective-csp-directives": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-invalid-boolean-attribute-value": "off",
    "unicorn/no-invalid-fetch-options": "error",
    "unicorn/no-invalid-file-input-accept": "error",
    "unicorn/no-invalid-integrity": "error",
    "unicorn/no-invalid-response-options": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-invalid-url-protocol-comparison": "off",
    // Targets JSON or Markdown, which ESLint does not lint here (Prettier formats them).
    "unicorn/no-javascript-url": "off",
    "unicorn/no-missing-local-resource": "error",
    "unicorn/no-shorthand-property-overrides": "error",
    "unicorn/no-transition-all": "error",
    "unicorn/no-unnecessary-fetch-options": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-url-in-search-params": "off",
    "unicorn/prefer-abort-signal-any": "error",
    "unicorn/prefer-abort-signal-timeout": "error",
    "unicorn/prefer-blob-reading-methods": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-location-assign": "off",
    "unicorn/prefer-response-static-json": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-url-can-parse": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-url-href": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-url-search-parameters": "off",
    "unicorn/relative-url-style": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/require-css-escape": "off",
    // Targets JSON or Markdown, which ESLint does not lint here (Prettier formats them).
    "unicorn/require-frontmatter-fields": "off",
    "unicorn/require-post-message-target-origin": "error",
  },
};
